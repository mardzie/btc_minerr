type BlockHeaderType = [u8; 80];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockHeaderBytes(BlockHeaderType);

impl BlockHeaderBytes {
    pub fn new(bytes: BlockHeaderType) -> Self {
        Self(bytes)
    }

    pub fn get_nonce(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(self.get_nonce_ref());

        u32::from_le_bytes(bytes)
    }

    pub fn set_nonce(&mut self, nonce: u32) {
        let mut nonce_bytes: [u8; 4] = nonce.to_le_bytes();
        self.get_nonce_ref().copy_from_slice(&mut nonce_bytes);
    }

    pub fn to_bytes(&self) -> BlockHeaderType {
        self.0.clone()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    fn get_nonce_ref(&mut self) -> &mut [u8] {
        &mut self.0[76..80]
    }
}

impl PartialEq<BlockHeaderType> for BlockHeaderBytes {
    fn eq(&self, other: &BlockHeaderType) -> bool {
        &self.0 == other
    }
}

impl PartialEq<BlockHeaderBytes> for BlockHeaderType {
    fn eq(&self, other: &BlockHeaderBytes) -> bool {
        self == &other.0
    }
}
