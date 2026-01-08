pub mod error;

mod block_header;
mod transaction;

pub use block_header::BlockHeader;
pub use transaction::{RawTransaction, Transaction};

#[derive(Debug)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<RawTransaction>,
}

impl Block {
    pub fn new(version: i32, prev_block_header_hash: [u8; 32], n_bits: u32, transactions: Vec<RawTransaction>) -> Self {
        let mut block = Self {
            header: BlockHeader::new(version, prev_block_header_hash, [0_u8; 32], n_bits),
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
    pub fn compute_merkle_root_hash(raw_transactions: &Vec<RawTransaction>) -> [u8; 32] {
        let mut hash = [0_u8; 32];
        
        todo!("Compute merkle root hash");
        
        hash.reverse();
        hash
    }
}
