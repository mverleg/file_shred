use ::aes_ctr::stream_cipher::generic_array::GenericArray;
use ::aes_ctr::stream_cipher::NewStreamCipher;
use ::aes_ctr::stream_cipher::SyncStreamCipher;
use ::aes_ctr::Aes256Ctr;

use crate::key::key::StretchKey;
use crate::key::Salt;
use crate::util::FedResult;

pub fn endec_aes256(mut data: Vec<u8>, key: &StretchKey, salt: &Salt) -> FedResult<Vec<u8>> {
    debug_assert!(key.key_data.unsecure().len() >= 32);
    debug_assert!(salt.salt.len() >= 16);
    let key = GenericArray::from_slice(&key.key_data.unsecure()[..32]);
    let nonce = GenericArray::from_slice(&salt.salt[..16]);
    let mut cipher = Aes256Ctr::new(&key, &nonce);
    cipher.apply_keystream(&mut data);
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aes256_reverse() {
        let key = StretchKey::mock_stretch(b"s3cr3t!");
        let salt = Salt::static_for_test(123_456_789);
        let input = vec![
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
            22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
            44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65,
            66, 67, 68, 69, 70,
        ];
        let secret = endec_aes256(input.clone(), &key, &salt).unwrap();
        let back = endec_aes256(secret, &key, &salt).unwrap();
        assert_eq!(back, input);
    }
}
