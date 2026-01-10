pub enum BtcMessage {
    
}

/// A Bitcoin message encoded into bytes.
///
/// Is used to as message to send to a Bitcoin node.
///
/// Implement [`ToBtcMessageBytes`] to use
#[derive(Debug, Clone, Hash)]
pub struct BtcMessageBytes(Vec<u8>);

impl BtcMessageBytes {
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

/// This trait provides a interface to convert a Bitcoin message to bytes.
pub trait ToBtcMessageBytes {
    fn to_btc_message_bytes(&self) -> BtcMessageBytes {
        unimplemented!("`to_btc_message_bytes` in `ToBtcMessageBytes` not implemented for message");
    }
}
