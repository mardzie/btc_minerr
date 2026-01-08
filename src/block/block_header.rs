use crate::{get_unix_timestamp, hash::Hash256};
use sha2::Digest;

/// # BlockHeader
///
/// Hashed are in internal byte order; the other values are all in little-endian order.
///
/// An example header in hex:
/// ```plaintext
/// 02000000 ........................... Block version: 2
///
/// b6ff0b1b1680a2862a30ca44d346d9e8
/// 910d334beb48ca0c0000000000000000 ... Hash of previous block's header
/// 9d10aa52ee949386ca9385695f04ede2
/// 70dda20810decd12bc9b048aaab31471 ... Merkle root
///
/// 24d95a54 ........................... [Unix time][unix epoch time]: 1415239972
/// 30c31b18 ........................... Target: 0x1bc330 * 256**(0x18-3)
/// fe9f0864 ........................... Nonce
/// ```
///
/// See [Block Header](https://learnmeabitcoin.com/technical/block/#header).
#[derive(Debug, Clone)]
pub struct BlockHeader {
    /// The version of this block header.#
    /// Indicates the validation rules.
    ///
    /// > little-endian
    pub version: i32,
    /// A SHA256(SHA256()) hash in internal byte order of the previous block´s header.
    ///
    /// > internal byte order; stored as bytes (not as hex)
    pub prev_block_header_hash: [u8; 32],
    /// A SHA256(SHA256()) hash in internal byte order.
    /// Its derived from the hashes of all transactions included in this block.
    ///
    /// # Construction
    ///
    /// The merkle root is constructed using all the TXIDs (Transaction IDs (hashes)) of transactions in this block.
    /// All TXIDs have to be in order as required by the consensus rules:
    /// - The coinbase transaction´s TXID is always placed first.
    ///   (The coinbase TXID refers to a unique identifier for the first transaction in a Bitcoin block, created by miners to distribute block rewards.)
    /// - Any input within this block can spend an output which also appears in this block (assuming the spend is otherwise valid).
    ///   However, the TXID corresponding to the output must be placed at some point before the TXID corresponding to the input.
    ///   This ensures that any program parsing block chain transactions linearly will encounter each output before it is used as an input.
    ///
    /// [Merkle Root](https://learnmeabitcoin.com/technical/block/merkle-root/)
    ///
    /// > internal byte order; stored as bytes (not as hex)
    pub merkle_root_hash: [u8; 32],
    /// The block time is a Unix epock time when the miner started hashing the header (according to the miner).
    /// Must be strictly greater than the median time of the previous 11 blocks.
    /// Full nodes will not accept blocks with headers more than two hours in the future according to their clock.
    ///
    /// > little-endian
    pub time: u32,
    /// An encoded version of the target threshold this block´s header hash must be less than or equal to.
    ///
    /// The target threshold is a 256-bit unsigned integer which a header hash must be equal to or below in order for that header to be a valid part of the block chain.
    /// However, the header file nBits provides only 32 bits of space,
    /// so the target number uses a less precise format called "compact" which works like a base-256 version of scientific notation:
    ///
    /// [Target](https://learnmeabitcoin.com/technical/mining/target/)
    ///
    /// > little-endian
    pub target: u32,
    /// An arbitrary number miners change to modify the header hash in order to produce a hash less than or equal to the target threshold.
    /// If all 32-bit values are tested, the time can be updated or the coinbase transaction can be changed and the merkle root updated.
    ///
    /// > little-endian
    pub nonce: u32,
}

impl BlockHeader {
    pub fn new(
        version: i32,
        prev_block_header_hash: [u8; 32],
        merkle_root_hash: [u8; 32],
        n_bits: u32,
    ) -> Self {
        Self {
            version,
            prev_block_header_hash,
            merkle_root_hash,
            time: get_unix_timestamp()
                .expect(&format!(
                    "Failed to get a valid Unix timestamp for new BlockHeader with previous block header hash: {}",
                    hex::encode(prev_block_header_hash)
                ))
                .as_secs() as u32,
            target: n_bits,
            nonce: Default::default(),
        }
    }

    /// Convert the [`BlockHeader`] into a valid byte array.

    pub fn as_bytes(&self) -> [u8; 80] {
        let mut bytes = [0_u8; 80];

        bytes[..4].copy_from_slice(&self.version.to_le_bytes());
        bytes[4..36].copy_from_slice(&self.prev_block_header_hash);
        bytes[36..68].copy_from_slice(&self.merkle_root_hash);
        bytes[68..72].copy_from_slice(&self.time.to_le_bytes());
        bytes[72..76].copy_from_slice(&self.target.to_le_bytes());
        bytes[76..80].copy_from_slice(&self.nonce.to_le_bytes());

        bytes
    }

    /// Hash the [`BlockHeader`] for mining.
    pub fn hash256(&self) -> [u8; 32] {
        Hash256::digest(&self.as_bytes())
    }

    pub fn get_target(&self) -> [u8; 256] {
        todo!()
    }
}

#[cfg(test)]
mod block_header_test {
    use super::BlockHeader;

    #[test]
    fn into_bytes_test() {
        let bytes = get_zeroed_block_header().as_bytes();

        assert_eq!([0_u8; 80], bytes);
    }

    #[test]
    fn hash256() {
        let header = get_zeroed_block_header().hash256();

        assert_eq!(
            [
                75, 231, 87, 14, 143, 112, 235, 9, 54, 64, 200, 70, 130, 116, 186, 117, 151, 69,
                167, 170, 43, 125, 37, 171, 30, 4, 33, 178, 89, 132, 80, 20
            ],
            header
        );
    }

    fn get_zeroed_block_header() -> BlockHeader {
        BlockHeader {
            version: 0,
            prev_block_header_hash: [0_u8; 32],
            merkle_root_hash: [0_u8; 32],
            time: 0,
            target: 0,
            nonce: 0,
        }
    }
}
