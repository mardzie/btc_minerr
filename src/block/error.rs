#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parse error occured: {0}")]
    TransactionParseError(String),
}