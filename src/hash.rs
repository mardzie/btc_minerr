use sha2::{Digest, Sha256};

pub struct Hash256;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hash {
    /// The natural byte order.
    ///
    /// The byte order as it comes out of the hash function.
    NaturalByte { hash: String },
    /// The reverse byte order.
    ///
    /// The byte order as shown on blockchain explorers.
    ReverseByte { hash: String },
}

impl Hash256 {
    /// The Hash256 algorithm for Bitcoin.
    pub fn digest(data: &[u8]) -> Hash {
        let generic_array_hash1 = Sha256::digest(data);
        let generic_array_hash2 = Sha256::digest(generic_array_hash1);
        let string_hash = hex::encode(generic_array_hash2);

        Hash::NaturalByte { hash: string_hash }
    }
}

impl Hash {
    pub fn from_natural_byte(hash: String) -> Self {
        Self::NaturalByte { hash }
    }

    pub fn from_natural_byte_str(hash: &str) -> Self {
        Self::NaturalByte {
            hash: hash.to_string(),
        }
    }

    pub fn from_reverse_byte(hash: String) -> Self {
        Self::NaturalByte { hash }
    }

    pub fn from_reverse_byte_str(hash: &str) -> Self {
        Self::ReverseByte {
            hash: hash.to_string(),
        }
    }

    pub fn to_natural_byte(self) -> Self {
        if let Self::ReverseByte { hash: old_hash } = self {
            let hash = Self::reverse_hex(&old_hash);

            Self::NaturalByte { hash }
        } else {
            self
        }
    }

    pub fn to_reverse_byte(self) -> Self {
        if let Self::NaturalByte { hash: old_hash } = self {
            let hash = Self::reverse_hex(&old_hash);

            Self::ReverseByte { hash }
        } else {
            self
        }
    }

    pub fn to_string(&self) -> String {
        self.inner().to_string()
    }

    pub fn as_str(&self) -> &str {
        self.inner()
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        Self::decode_to_bytes(self.inner())
    }

    /// The checksum of this [`Hash`].
    ///
    /// The checksum are the first 4 chars from a hash.
    ///
    /// See [Checksum](https://learnmeabitcoin.com/technical/keys/checksum/).
    pub fn checksum(&self) -> [u8; 4] {
        let bytes = match self {
            Self::NaturalByte { hash } => Self::decode_to_bytes(hash),
            Self::ReverseByte { hash } => {
                let reversed = Self::reverse_hex(hash);
                Self::decode_to_bytes(&reversed)
            }
        };
        let mut checksum = [0u8; 4];
        checksum.copy_from_slice(&bytes[..4]);

        checksum
    }

    /// Check checksums.
    pub fn check(&self, checksum: &[u8; 4]) -> bool {
        &self.checksum() == checksum
    }

    pub fn inner(&self) -> &str {
        match self {
            Self::NaturalByte { hash } => hash,
            Self::ReverseByte { hash } => hash,
        }
    }

    fn decode_to_bytes(hex: &str) -> [u8; 32] {
        let mut out = [0u8; 32];
        hex::decode_to_slice(hex, &mut out)
            .expect("Failed to decode hash into bytes: Malformed hash");

        out
    }

    fn reverse_hex(hex: &str) -> String {
        hex.chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .rev()
            .flat_map(|pair| {
                if pair.len() == 2 {
                    pair.to_vec()
                } else {
                    panic!("Failed to reverse hex String: Invalid hex String");
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod hash_test {
    use super::Hash256;

    #[test]
    fn hash256() {
        let data = b"My cool String!";
        let hash = Hash256::digest(data);
        assert_eq!(
            "ef7391fd5ad3916f2e1c9d9df3b5e2adc546f63509c04ed9ec010cc880c96045",
            hash.as_str()
        );
    }

    #[test]
    fn reverse_hex() {
        let data = b"My cool String!";
        let hash = Hash256::digest(data);
    }

    #[test]
    fn checksum() {
        let data = b"My awesome and blazingly fast str";
        let hash = Hash256::digest(data);

        let checksum = [0xC6, 0x2A, 0x07, 0xDB];
        assert_eq!(checksum, hash.checksum());
        assert!(hash.check(&checksum));
    }
}
