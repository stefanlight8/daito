use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_nonce() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .to_string()
}
