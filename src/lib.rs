use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_unix() -> u64{
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("get_current_unix_err").as_secs()
}
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
