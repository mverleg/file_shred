use ::std::convert::TryInto;

use ::data_encoding::BASE64URL_NOPAD;

pub type FedResult<T> = Result<T, String>;

pub fn u64_to_base64str(value: u64) -> String {
    BASE64URL_NOPAD.encode(&value.to_le_bytes())
}

pub fn base64str_to_u64(base64_str: &str) -> FedResult<u64> {
    let bytes: &[u8] = &BASE64URL_NOPAD.decode(base64_str.as_bytes()).unwrap();
    Ok(u64::from_le_bytes(match bytes.try_into() {
        Ok(nr) => nr,
        Err(_) => return Err(format!("could not decode '{}' to a number", base64_str))
    }))
}

#[cfg(test)]
mod data_base_encoding {
    use super::u64_to_base64str;
    use super::base64str_to_u64;

    #[test]
    fn base() {
        let original: u64 = 123_456_789_000;
        let encoded = u64_to_base64str(123_456_789_000);
        let back = base64str_to_u64(&encoded).unwrap();
        assert_eq!(back, original);
    }
}
