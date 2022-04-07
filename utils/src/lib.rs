//! this is a utils lib;
use crypto::digest::Digest;
use crypto::md5::Md5;
/// this is func to comput md5 value
/// #example
/// ```
/// use utils::md5;
/// md5("123456".to_string())
/// ```
pub fn md5<S:Into<String>>(input:S)->String{
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}

#[test]
fn md5_test(){
    println!("{}",md5("123456"));
    assert_eq!(md5("123456"),"e10adc3949ba59abbe56e057f20f883e");
}
