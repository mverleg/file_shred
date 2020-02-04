use ::std::convert::TryInto;
use ::std::io;

use ::data_encoding::BASE64URL_NOPAD;

pub type FedResult<T> = Result<T, String>;

pub fn wrap_io<T>(res: io::Result<T>) -> FedResult<T> {
    match res {
        Ok(val) => FedResult::Ok(val),
        Err(val) => FedResult::Err(format!("{}", val)),
    }
}

pub fn u64_to_base64str(value: u64) -> String {
    BASE64URL_NOPAD.encode(&value.to_le_bytes())
}

pub fn base64str_to_u64(base64_str: &str) -> FedResult<u64> {
    let bytes = match BASE64URL_NOPAD.decode(base64_str.as_bytes()) {
        Ok(bytes) => bytes,
        Err(err) => return Err("did not find valid base64 encoded integer (expecting url base characters)".to_owned()),
    };
    Ok(u64::from_le_bytes(match bytes.as_slice().try_into() {
        Ok(nr) => nr,
        Err(_) => return Err(format!("could not decode '{}' to a number", base64_str))
    }))
}

pub fn u8s_to_base64str(value: &[u8]) -> String {
    BASE64URL_NOPAD.encode(value)
}

//TODO @mark: tests
pub fn base64str_to_u8s(base64_str: &str) -> FedResult<Vec<u8>> {
    match BASE64URL_NOPAD.decode(base64_str.as_bytes()) {
        Ok(bytes) => Ok(bytes),
        Err(err) => Err("did not find valid base64 content (expecting url base characters)".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::base64str_to_u64;
    use super::u64_to_base64str;
    use super::base64str_to_u8s;
    use super::u8s_to_base64str;

    #[test]
    fn base_u64() {
        let original: u64 = 123_456_789_000;
        let encoded = u64_to_base64str(original);
        let back = base64str_to_u64(&encoded).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn base_u8s() {
        let original: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 3, 2, 1];
        let encoded = u8s_to_base64str(&original);
        let back = base64str_to_u8s(&encoded).unwrap();
        assert_eq!(back, original);
    }
}
