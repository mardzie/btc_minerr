use crate::{
    hash::Hash,
    networking::{NetworkType, command::Command, traits::NetworkInformation},
};

#[derive(Debug, Clone, Hash)]
pub struct Header {
    magic_bytes: NetworkType,
    command: Command,
    size: u32,
    checksum: [u8; 4],
}

impl Header {
    pub fn from_bytes(
        magic_bytes: &[u8; 4],
        command: &[u8; 12],
        size: u32,
        checksum: [u8; 4],
    ) -> Self {
        let magic_bytes = NetworkType::from_magic_bytes(magic_bytes);
        let command = Command::from_bytes(command);

        Self {
            magic_bytes,
            command,
            size,
            checksum,
        }
    }

    /// Check the payload.
    pub fn check_payload(&self, payload_hash: &Hash) -> bool {
        payload_hash.check(&self.checksum)
    }

    pub fn network_type(&self) -> NetworkType {
        self.magic_bytes
    }

    pub fn command(&self) -> Command {
        self.command
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn checksum(&self) -> [u8; 4] {
        self.checksum
    }

    pub fn to_bytes(self) -> [u8; 24] {
        let mut bytes = [0u8; 24];
        bytes[..4].copy_from_slice(&self.magic_bytes.magic_bytes());
        bytes[4..16].copy_from_slice(&self.command.to_bytes());
        bytes[16..20].copy_from_slice(&self.size.to_le_bytes());
        bytes[20..24].copy_from_slice(&self.checksum);

        bytes
    }
}
