use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{self, ToSocketAddrs},
    sync::{Arc, Mutex},
    thread,
};

#[allow(unused_imports)]
pub use crate::networking::message::{BtcMessage, BtcMessageBytes};

mod error;
mod message;

pub const PROTOCOL_VERSION: u32 = 70015;
pub const MAGIC_BYTES_MAINNET: u32 = 0xF9BEB4D9;
pub const MAGIC_BYTES_REGTEST: u32 = 0xFABFB5DA;
pub const MAGIC_BYTES_TESTNET3: u32 = 0x0B110907;

type ArcMutex<T> = Arc<Mutex<T>>;
type MagicBytes = [u8; 4];
type CommandBytes = [u8; 12];
type SizeBytes = [u8; 4];
type ChecksumBytes = [u8; 4];

pub struct Network {
    recv_queue: ArcMutex<VecDeque<BtcMessage>>,
    send_queue: ArcMutex<VecDeque<BtcMessageBytes>>,

    read_worker: thread::JoinHandle<()>,
    write_worker: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NetworkType {
    Mainnet,
    Testnet,
    Regtest,
}

pub trait NetworkInformation {
    fn port(&self) -> u16;

    /// The same as [`magic_number`] only as bytes in little-endian.
    fn magic_bytes(&self) -> [u8; 4];

    /// The same as [`magic_bytes`] only as a `u32` in little-endian.
    fn magic_number(&self) -> u32;
}

impl Network {
    /// Create a Network and connect to an address.
    pub fn connect<A>(addr: A, net_type: NetworkType) -> Result<Self, error::Error>
    where
        A: ToSocketAddrs,
    {
        log::info!("Connecting...");

        let read_stream = net::TcpStream::connect(addr)?;
        let write_stream = read_stream.try_clone().expect("Failed to clone TcpStream");

        let recv_queue = Arc::new(Mutex::new(VecDeque::with_capacity(16)));
        let send_queue = Arc::new(Mutex::new(VecDeque::with_capacity(16)));

        let recf_queue_c = recv_queue.clone();
        let read_worker = thread::spawn(move || Self::read_worker(read_stream, recf_queue_c));

        let send_queue_c = send_queue.clone();
        let write_worker = thread::spawn(move || Self::write_worker(write_stream, send_queue_c));

        Ok(Self {
            recv_queue,
            send_queue,

            read_worker,
            write_worker,
        })
    }

    fn handshake(mut read_stream: &net::TcpStream) -> Result<(), error::Error> {
        todo!();
    }

    fn read_worker(mut read_stream: net::TcpStream, recv_queue: ArcMutex<VecDeque<BtcMessage>>) {
        Self::handshake(&read_stream).expect("Failed to finish handshake.");

        let mut magic_bytes: MagicBytes = [0u8; 4];
        let mut command_bytes: CommandBytes = [0u8; 12];
        let mut size_bytes: SizeBytes = [0u8; 4];
        let mut checksum_bytes: ChecksumBytes = [0u8; 4];
        let mut size: u32;
        let mut payload: Vec<u8> = Vec::new();

        loop {
            read_stream
                .read_exact(&mut magic_bytes)
                .expect("Failed to read magic bytes.");
            read_stream
                .read_exact(&mut command_bytes)
                .expect("Failed to read command.");
            read_stream
                .read_exact(&mut size_bytes)
                .expect("Failed to read size.");
            read_stream
                .read_exact(&mut checksum_bytes)
                .expect("Failed to read checksum.");

            size = u32::from_le_bytes(size_bytes);
            payload.resize(size as usize, 0);
            read_stream
                .read_exact(&mut payload)
                .expect("Failed to read payload.");

            Self::process_payload(
                &recv_queue,
                &magic_bytes,
                &command_bytes,
                size,
                &checksum_bytes,
                &payload,
            );
        }
    }

    fn process_payload(
        recv_queue: &ArcMutex<VecDeque<BtcMessage>>,
        magic_bytes: &MagicBytes,
        command_bytes: &CommandBytes,
        size: u32,
        checksum_bytes: &ChecksumBytes,
        payload: &[u8],
    ) {
        todo!("Process payload")
    }

    fn write_worker(
        mut write_stream: net::TcpStream,
        send_queue: ArcMutex<VecDeque<BtcMessageBytes>>,
    ) {
        loop {
            if let Some(msg) = send_queue
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .pop_front()
            {
                match write_stream.write_all(msg.bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("Failed to write message: {}", e);
                    }
                };
            };
        }
    }

    /// Push the message onto the sending queue.
    /// This message will be sent as soon as every message before it was sent.
    ///
    /// This operates on a FIFO (first-in-first-out) queue.
    pub fn send(&mut self, message: BtcMessage) {
        self.send_queue
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push_back(message.to_message_bytes());
    }

    /// Get the oldest received unread [`BtcMessage`].
    ///
    /// This operates on a FIFO (first-in-first-out) queue.
    pub fn recv(&self) -> Option<BtcMessage> {
        self.recv_queue
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .pop_front()
    }

    /// Get the current received count of [`BtcMessage`].
    ///
    /// This can always change and the count of messages may be outdated directly after this function returns.
    pub fn recvd_queue_len(&self) -> usize {
        self.recv_queue
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .len()
    }
}

impl NetworkInformation for NetworkType {
    fn port(&self) -> u16 {
        match self {
            Self::Mainnet => 8333,
            Self::Regtest => 18333,
            Self::Testnet => 18444,
        }
    }

    fn magic_bytes(&self) -> [u8; 4] {
        // The magic number is in littel-endian. We do not want to convert it again.
        self.magic_number().to_ne_bytes()
    }

    fn magic_number(&self) -> u32 {
        match self {
            Self::Mainnet => MAGIC_BYTES_MAINNET,
            Self::Regtest => MAGIC_BYTES_REGTEST,
            Self::Testnet => MAGIC_BYTES_TESTNET3,
        }
    }
}
