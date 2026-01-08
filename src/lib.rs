mod block;
mod minerr;
mod hash;

pub fn get_unix_timestamp() -> Result<std::time::Duration, std::time::SystemTimeError> {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now().duration_since(UNIX_EPOCH)
}
