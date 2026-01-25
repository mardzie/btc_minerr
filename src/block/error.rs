#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("the Merkle root hash is required but is not available. Please call")]
    MerkleRootHashRequired,
}
