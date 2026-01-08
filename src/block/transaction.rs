use super::error::Error;

#[derive(Debug, Clone)]
pub struct Transaction {}

#[derive(Debug, Clone)]
pub struct RawTransaction {
    pub data: Vec<u8>,
}

impl Transaction {}

impl RawTransaction {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl TryFrom<RawTransaction> for Transaction {
    type Error = Error;

    fn try_from(raw_transaction: RawTransaction) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<Transaction> for RawTransaction {
    type Error = Error;

    fn try_from(transaction: Transaction) -> Result<Self, Self::Error> {
        todo!()
    }
}
