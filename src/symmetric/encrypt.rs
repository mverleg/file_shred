
use ::aes::Aes256;
use ::aes::block_cipher_trait::BlockCipher;
use ::aes::block_cipher_trait::generic_array::GenericArray;

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

pub fn encrypt_aes256(mut data: Vec<u8>, key: &StretchKey, salt: &Salt) -> FedResult<Vec<u8>> {
    let key_ga = GenericArray::clone_from_slice(&key.key_data.unsecure());
    let cipher = Aes256::new(&key_ga);
    //cipher.encrypt_block(&data);
    unimplemented!()
}

pub fn encrypt_blowfish(mut data: Vec<u8>, key: &StretchKey) -> FedResult<Vec<u8>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aes256_small() {
        let key = StretchKey::new("s3cr3t!".as_bytes());
        let salt = Salt::static_for_test(123_456_789);
        let input = vec![
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
            64, 65, 66, 67, 68, 69, 70,
        ];
        let actual = encrypt_aes256(input, &key, &salt).unwrap();
        let expected: Vec<u8> = vec![];
        assert_eq!(actual, expected);
    }

    //TODO @mark: more tests
}
