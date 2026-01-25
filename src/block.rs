pub mod error;

mod block_header;
mod transaction;

pub use block_header::BlockHeader;
pub use transaction::{RawTransaction, Transaction};

use crate::hash::{Hash, Hash256};

#[derive(Debug)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<RawTransaction>,
}

impl Block {
    pub fn new(
        version: i32,
        prev_block_header_hash: Hash,
        n_bits: u32,
        transactions: Vec<RawTransaction>,
    ) -> Self {
        let mut block = Self {
            header: BlockHeader::new(
                version,
                prev_block_header_hash,
                Self::compute_merkle_root_hash(&transactions),
                n_bits,
            ),
            transactions,
        };
        block.update_merkle_root_hash();

        block
    }

    /// Compute the merkle hash and apply it into the internal [`BlockHeader`].
    pub fn update_merkle_root_hash(&mut self) {
        self.header.merkle_root_hash = Self::compute_merkle_root_hash(&self.transactions);
    }

    /// Computes the merkle hash.
    ///
    /// The merkle root hash is in reverse byte order.
    pub fn compute_merkle_root_hash(raw_transactions: &Vec<RawTransaction>) -> Hash {
        let mut hash = String::new();

        if raw_transactions.len() == 1 {}

        todo!();

        let mut merkle_root_hash = [0u8; 16];
        hex::encode_to_slice(hash, &mut merkle_root_hash);

        Hash::NaturalByte { hash }
    }

    fn compute_merkle_branch(txid1: Hash, txid2: Hash) -> Hash {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(&txid1.to_natural_byte().to_bytes());
        bytes[32..].copy_from_slice(&txid2.to_natural_byte().to_bytes());
        Hash256::digest(&bytes)
    }

    pub fn block_header_hash256(&self) -> Hash {
        todo!()
    }
}
