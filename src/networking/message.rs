use crate::networking::{header::Header, payload::Payload, traits::FromToIpV6};

#[derive(Debug, Clone, Hash)]
pub struct Message {
    header: Header,
    payload: Payload,
}

/// A Bitcoin message encoded into bytes.
///
/// Is used to as message to send to a Bitcoin node.
///
/// Implement [`ToBtcMessageBytes`] to use
#[derive(Debug, Clone, Hash)]
pub struct MessageBytes(pub Vec<u8>);

impl Message {
    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn payload(&self) -> &Payload {
        &self.payload
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut payload = self.payload.to_bytes();
        payload.reserve_exact(24);
        let len = payload.len();
        // Make space for header.
        payload.copy_within(..len, 24);
        payload[..24].copy_from_slice(&self.header.to_bytes());
        payload.shrink_to_fit();

        payload
    }
}

impl MessageBytes {
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

impl FromToIpV6 for std::net::IpAddr {
    fn to_v6(self) -> std::net::Ipv6Addr {
        match self {
            Self::V4(addr) => addr.to_ipv6_mapped(),
            Self::V6(addr) => addr,
        }
    }

    fn from_be_bytes(bytes: [u8; 16]) -> Self {
        std::net::IpAddr::V6(std::net::Ipv6Addr::from_bits(u128::from_be_bytes(bytes)))
    }
}
