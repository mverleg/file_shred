use ::aes_ctr::Aes256Ctr;
use ::aes_ctr::stream_cipher::generic_array::GenericArray;
use ::aes_ctr::stream_cipher::NewStreamCipher;
use ::aes_ctr::stream_cipher::SyncStreamCipher;
use ::aes_ctr::stream_cipher::SyncStreamCipherSeek;

use crate::header::SymmetricEncryptionAlg;
use crate::key::key::StretchKey;
use crate::key::Salt;
use crate::util::FedResult;

pub fn encrypt_file(mut data: Vec<u8>, key: &StretchKey, salt: &Salt, encrypt_algs: &[SymmetricEncryptionAlg]) -> FedResult<Vec<u8>> {
    assert!(encrypt_algs.len() >= 1);
    for encrypt_alg in encrypt_algs {
        data = match encrypt_alg {
            SymmetricEncryptionAlg::Aes256 => encrypt_aes256(data, key, salt)?,
            SymmetricEncryptionAlg::Blowfish => encrypt_blowfish(data, key)?,
        }
    }
    Ok(data)
}

//TODO @mark: this is both encrypt and decrypt
pub fn encrypt_aes256(mut data: Vec<u8>, key: &StretchKey, salt: &Salt) -> FedResult<Vec<u8>> {
    debug_assert!(key.key_data.unsecure().len() >= 32);
    debug_assert!(salt.salt.len() >= 16);
    let key = GenericArray::from_slice(&key.key_data.unsecure()[..32]);
    let nonce = GenericArray::from_slice(&salt.salt[..16]);
    let mut cipher = Aes256Ctr::new(&key, &nonce);
    cipher.apply_keystream(&mut data);
    Ok(data)
}

pub fn encrypt_blowfish(mut data: Vec<u8>, key: &StretchKey) -> FedResult<Vec<u8>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aes_ctr::Aes128Ctr;
    use crate::key::hash::fastish_hash;

    #[test]
    fn aes_ctr_sanity_check_demo() {
        let mut data = [1, 2, 3, 4, 5, 6, 7];
        let key = GenericArray::clone_from_slice(b"very secret key.");
        let nonce = GenericArray::clone_from_slice(b"and secret nonce");
        let mut cipher = Aes128Ctr::new(&key, &nonce);
        cipher.apply_keystream(&mut data);
        assert_eq!(data, [6, 245, 126, 124, 180, 146, 37]);
        cipher.seek(0);
        cipher.apply_keystream(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5, 6, 7]);
    }

    //TODO @mark: test nonce and key different length

    #[test]
    fn aes_ctr_sanity_check_own() {
        let mut input = vec![
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
            64, 65, 66, 67, 68, 69, 70,
        ];
        let raw_key = fastish_hash(b"s3cr3t!");
        let raw_nonce = fastish_hash(b"n0nc3");
        let key = GenericArray::from_slice(&raw_key[..32]);
        let nonce = GenericArray::from_slice(&raw_nonce[..16]);
        let mut cipher = Aes256Ctr::new(&key, &nonce);
        cipher.apply_keystream(&mut input);
    }

    #[test]
    fn aes256_small() {
        let key = StretchKey::mock_stretch("s3cr3t!".as_bytes());
        let salt = Salt::static_for_test(123_456_789);
        let input = vec![
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
            64, 65, 66, 67, 68, 69, 70,
        ];
        let actual = encrypt_aes256(input, &key, &salt).unwrap();
        let expected: Vec<u8> = vec![17, 154, 31, 230, 44, 192, 227, 243,
            227, 255, 149, 126, 201, 220, 251, 132, 219, 73, 148, 126, 63,
            78, 9, 183, 92, 130, 232, 146, 102, 241, 230, 77, 71, 87, 16,
            59, 74, 232, 36, 39, 65, 134, 137, 244, 105, 165, 16, 151, 233,
            85, 60, 40, 168, 116, 45, 121, 23, 224, 34, 33, 78, 203, 137,
            199, 157, 211, 173, 61, 101, 153, 139];
        assert_eq!(actual, expected);
    }

    //TODO @mark: more tests
}
