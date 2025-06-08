use std::time::{SystemTime, UNIX_EPOCH};

pub trait Now {
    fn now_unix() -> u64;
}

pub struct SystemNow;

impl Now for SystemNow {
    fn now_unix() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| panic!("Time clock went backwards, can not continue"))
            .as_secs()
    }
}
