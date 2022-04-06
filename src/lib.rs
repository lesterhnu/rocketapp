use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("get_current_unix_err").as_secs()
}

pub fn add<T:Add<Output=T>>(x: T, y: T) -> T {
    x + y
}

#[test]
pub fn testadd() {
    assert_eq!(add(1, 2), 3);
}
