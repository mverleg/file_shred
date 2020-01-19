use ::std::convert::TryInto;

use ::data_encoding::BASE64URL_NOPAD;

pub type FedResult<T> = Result<T, String>;

pub fn u64_to_base64str(value: u64) -> String {
    BASE64URL_NOPAD.encode(&value.to_le_bytes())
}

pub fn base64str_to_u64(base64_str: &str) -> u64 {
    let bytes: &[u8] = &BASE64URL_NOPAD.decode(base64_str.as_bytes()).unwrap();
    u64::from_le_bytes(bytes.try_into().unwrap())
}
