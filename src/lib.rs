use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};


pub fn get_current_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("get_current_unix_err").as_secs()
}

pub fn add<T: Add<Output=T>>(x: T, y: T) -> T {
    x + y
}

#[test]
fn add_test() {
    assert_eq!(add(1, 2), 3);
}

pub fn md5() {
    let s = utils::md5("123".to_string());
    println!("{}", s)
}

#[test]
pub fn get_unix() {
    println!("{:?}", time::OffsetDateTime::now_utc().unix_timestamp());
}