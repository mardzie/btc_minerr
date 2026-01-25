use std::{
    collections::VecDeque,
    io::{Read, Write},
    net::{self, ToSocketAddrs},
    sync::{Arc, Mutex},
    thread,
};

#[allow(unused_imports)]
pub use crate::networking::message::MessageBytes;
use crate::networking::{header::Header, message::Message, payload::Payload, traits::NetworkInformation};

mod command;
mod error;
mod header;
mod message;
mod payload;
mod traits;

pub const PROTOCOL_VERSION: u32 = 70015;
pub const MAGIC_NUMBER_MAINNET: u32 = 0xF9BEB4D9;
pub const MAGIC_NUMBER_REGTEST: u32 = 0xFABFB5DA;
pub const MAGIC_NUMBER_TESTNET3: u32 = 0x0B110907;

type ArcMutex<T> = Arc<Mutex<T>>;
type MagicBytes = [u8; 4];
type CommandBytes = [u8; 12];
type SizeBytes = [u8; 4];
type ChecksumBytes = [u8; 4];

pub struct Network {
    recv_queue: ArcMutex<VecDeque<Message>>,
    send_queue: ArcMutex<VecDeque<Vec<u8>>>,

    read_worker: thread::JoinHandle<()>,
    write_worker: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetworkType {
    Mainnet,
    Testnet,
    Regtest,
}

impl Network {
    /// Create a Network and connect to an address.
    pub fn connect<A>(addr: A, net_type: impl NetworkInformation) -> Result<Self, error::Error>
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

    fn read_worker(mut read_stream: net::TcpStream, recv_queue: ArcMutex<VecDeque<Message>>) {
        Self::handshake(&read_stream).expect("Failed to finish handshake.");

        let mut header_bytes = [0u8; 24];
        let mut payload: Vec<u8> = Vec::new();

        loop {
            read_stream.read_exact(&mut header_bytes).expect("Failed to read header.");
            let header = Header::from_bytes(&header_bytes);
            
            payload.resize(header.size() as usize, 0);
            read_stream
                .read_exact(&mut payload)
                .expect("Failed to read payload.");
            

            Self::process_payload(
                &recv_queue,
                header,
                &payload,
            );
        }
    }

    fn process_payload(
        recv_queue: &ArcMutex<VecDeque<Message>>,
        header: Header,
        payload: &[u8],
    ) {
        todo!("Process payload")
    }

    fn write_worker(
        mut write_stream: net::TcpStream,
        send_queue: ArcMutex<VecDeque<Vec<u8>>>,
    ) {
        loop {
            if let Some(msg) = send_queue
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .pop_front()
            {
                match write_stream.write_all(&msg) {
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
    pub fn send(&mut self, message: Message) {
        self.send_queue
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push_back(message.to_bytes());
    }

    /// Get the oldest received unread [`BtcMessage`].
    ///
    /// This operates on a FIFO (first-in-first-out) queue.
    pub fn recv(&self) -> Option<Message> {
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

impl NetworkType {
    pub fn from_magic_bytes(magic_bytes: &[u8; 4]) -> Self {
        Self::from_magic_number(u32::from_ne_bytes(*magic_bytes))
    }

    pub fn from_magic_number(magic_number: u32) -> Self {
        match magic_number {
            MAGIC_NUMBER_MAINNET => Self::Mainnet,
            MAGIC_NUMBER_REGTEST => Self::Regtest,
            MAGIC_NUMBER_TESTNET3 => Self::Testnet,
            _ => panic!("Unknown magic number!"),
        }
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
            Self::Mainnet => MAGIC_NUMBER_MAINNET,
            Self::Regtest => MAGIC_NUMBER_REGTEST,
            Self::Testnet => MAGIC_NUMBER_TESTNET3,
        }
    }
}
