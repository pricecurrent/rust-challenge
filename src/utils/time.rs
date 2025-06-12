use std::time::{SystemTime, UNIX_EPOCH};

pub trait Now {
    fn now_unix() -> u64;
}

pub struct SystemNow;

impl Now for SystemNow {
    fn now_unix() -> u64 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }
}
