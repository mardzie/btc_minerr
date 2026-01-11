use std::net;

use crate::{get_unix_timestamp, networking::PROTOCOL_VERSION};

pub enum BtcMessage {
    Version {
        // version: u32,
        /// A bit field.
        services: u64,
        // time: u64,
        /// A bit field.
        remote_services: u64,
        /// remote_addr is for address and port.
        remote_addr: net::SocketAddr,
        /// A bit field in little-endian.
        local_services: u64,
        /// local_addr is for address and port.
        local_addr: net::SocketAddr,
        /// The nonce.
        nonce: u64,
        /// user_agent is in [compact size](https://learnmeabitcoin.com/technical/general/compact-size/) in ascii format.
        user_agent: String,
        ///The latest block of our blockchain.
        last_block: u32,
    },
    Verack,
}

/// A Bitcoin message encoded into bytes.
///
/// Is used to as message to send to a Bitcoin node.
///
/// Implement [`ToBtcMessageBytes`] to use
#[derive(Debug, Clone, Hash)]
pub struct BtcMessageBytes(Vec<u8>);

pub trait ToIpV6 {
    fn to_v6(self) -> net::Ipv6Addr;
}

impl BtcMessageBytes {
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

impl BtcMessage {
    pub fn new_version(remote_addr: net::SocketAddr, local_addr: net::SocketAddr) -> Self {
        Self::Version {
            services: 0,
            remote_services: 0,
            remote_addr,
            local_services: 0,
            local_addr,
            nonce: 0,
            user_agent: String::new(),
            last_block: 0,
        }
    }

    pub fn to_message_bytes(self) -> BtcMessageBytes {
        let bytes: Vec<u8> = match self {
            Self::Version {
                services,
                remote_services,
                remote_addr,
                local_services,
                local_addr,
                nonce,
                user_agent,
                last_block,
            } => {
                /// Size in bytes of version. Excludes `user_agent` because of dynamic size.
                const SIZE_OF_VERSION: usize = 84;
                
                if !user_agent.is_ascii() {
                    let err_msg = format!("Failed to convert `BtcMessage::Version` into `BtcMessageBytes`. user_agent not a valid ASCII String!");
                    log::error!("{}", err_msg);
                    panic!("{}", err_msg);
                };
                
                let user_agent_bytes = if user_agent.is_empty() {
                    Vec::from([0])
                } else {
                    user_agent.into_bytes()
                };

                let mut bytes = Vec::with_capacity(SIZE_OF_VERSION + user_agent_bytes.len());

                bytes[..4].copy_from_slice(&PROTOCOL_VERSION.to_le_bytes());
                bytes[4..12].copy_from_slice(&services.to_le_bytes());
                bytes[12..20].copy_from_slice(
                    &get_unix_timestamp()
                        .expect("Invalid unix timestamp while converting a BtcMessage to bytes.")
                        .as_secs()
                        .to_le_bytes(),
                );
                bytes[20..28].copy_from_slice(&remote_services.to_le_bytes());
                bytes[28..44].copy_from_slice(&remote_addr.ip().to_v6().to_bits().to_be_bytes());
                bytes[44..46].copy_from_slice(&remote_addr.port().to_be_bytes());
                bytes[46..54].copy_from_slice(&local_services.to_le_bytes());
                bytes[54..70].copy_from_slice(&local_addr.ip().to_v6().to_bits().to_be_bytes());
                bytes[70..72].copy_from_slice(&local_addr.port().to_be_bytes());
                bytes[72..80].copy_from_slice(&nonce.to_le_bytes());
                let usr_agnt_end_idx = 80 + user_agent_bytes.len();
                bytes[80..usr_agnt_end_idx].copy_from_slice(&user_agent_bytes);
                bytes[usr_agnt_end_idx..usr_agnt_end_idx + 4]
                    .copy_from_slice(&last_block.to_le_bytes());

                bytes
            }
            Self::Verack => {
                todo!()
            }
        };

        BtcMessageBytes(bytes)
    }
}

impl ToIpV6 for net::IpAddr {
    fn to_v6(self) -> net::Ipv6Addr {
        match self {
            Self::V4(addr) => addr.to_ipv6_mapped(),
            Self::V6(addr) => addr,
        }
    }
}
