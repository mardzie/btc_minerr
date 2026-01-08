use sha2::{Digest, Sha256};

pub struct Hash256;

impl Hash256 {
    /// The Hash256 algorithm for Bitcoin.
    pub fn digest(data: &[u8]) -> [u8; 32] {
        let generic_array_hash1 = Sha256::digest(data);
        let generic_array_hash2 = Sha256::digest(generic_array_hash1);
        let mut hash = [0_u8; 32];
        hash.copy_from_slice(&generic_array_hash2);

        hash
    }
}
