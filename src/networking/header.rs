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
    /// Takes exactly 24 bytes.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut magic_bytes = [0u8; 4];
        let mut command_bytes = [0u8; 12];
        let mut size_bytes = [0u8; 4];
        let mut checksum = [0u8; 4];
        
        magic_bytes.copy_from_slice(&bytes[..4]);
        command_bytes.copy_from_slice(&bytes[4..16]);
        size_bytes.copy_from_slice(&bytes[16..20]);
        checksum.copy_from_slice(&bytes[20..]);
        
        let magic_bytes = NetworkType::from_magic_bytes(&magic_bytes);
        let command = Command::from_bytes(&command_bytes);
        
        Self {
            magic_bytes,
            command,
            size: u32::from_be_bytes(size_bytes),
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
