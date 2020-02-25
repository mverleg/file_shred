use ::block_modes::BlockMode;

use crate::header::SymmetricEncryptionAlg;
use crate::key::key::StretchKey;
use crate::key::Salt;
use crate::symmetric::{Aes256Cbc, TwofishCbc};
use crate::util::FedResult;

pub fn decrypt_file(
    mut data: Vec<u8>,
    key: &StretchKey,
    salt: &Salt,
    encrypt_algs: &[SymmetricEncryptionAlg],
) -> FedResult<Vec<u8>> {
    assert!(!encrypt_algs.is_empty());
    for encrypt_alg in encrypt_algs {
        data = match encrypt_alg {
            SymmetricEncryptionAlg::Aes256 => decrypt_aes256(&data, key, salt)?,
            SymmetricEncryptionAlg::Twofish => decrypt_twofish(&data, key, salt)?,
        }
    }
    Ok(data)
}

pub fn decrypt_aes256(data: &[u8], key: &StretchKey, salt: &Salt) -> FedResult<Vec<u8>> {
    debug_assert!(key.key_data.unsecure().len() >= 32);
    debug_assert!(salt.salt.len() >= 16);
    let cipher = Aes256Cbc::new_var(&key.key_data.unsecure()[..32], &salt.salt[..16]).unwrap();
    match cipher.decrypt_vec(data) {
        Ok(plain) => Ok(plain),
        Err(err) => Err(format!("Decryption algorithm failed: {}", err)),
    }
}

pub fn decrypt_twofish(data: &[u8], key: &StretchKey, salt: &Salt) -> FedResult<Vec<u8>> {
    debug_assert!(key.key_data.unsecure().len() >= 16);
    debug_assert!(salt.salt.len() >= 16);
    let cipher = TwofishCbc::new_var(&key.key_data.unsecure()[..16], &salt.salt[..16]).unwrap();
    match cipher.decrypt_vec(data) {
        Ok(plain) => Ok(plain),
        Err(err) => Err(format!("Decryption algorithm failed: {}", err)),
    }
}

#[cfg(test)]
mod tests {
    use crate::files::mockfile::generate_test_file_content_for_test;
    use crate::symmetric::encrypt::{encrypt_aes256, encrypt_twofish};

    use super::*;

    #[test]
    fn aes256_small() {
        let key = StretchKey::mock_stretch(b"s3cr3t!");
        let salt = Salt::fixed_for_test(123_456_789);
        let input: Vec<u8> = vec![
            8, 161, 111, 221, 11, 228, 30, 113, 127, 148, 186, 160, 217, 32, 132, 212, 24, 230, 13,
            196, 126, 21, 244, 203, 34, 121, 157, 181, 3, 37, 201, 196, 21, 217, 132, 207, 61, 60,
            74, 108, 41, 135, 28, 68, 53, 129, 76, 135, 250, 215, 77, 32, 13, 150, 85, 84, 172,
            159, 42, 60, 228, 3, 21, 221, 83, 195, 0, 15, 124, 67, 62, 219, 72, 251, 230, 81, 87,
            117, 239, 90,
        ];
        let actual = decrypt_aes256(&input, &key, &salt).unwrap();
        let expected = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70,
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn aes256_empty() {
        let key = StretchKey::mock_stretch(b"s3cr3t!");
        let salt = Salt::fixed_for_test(111_555_999);
        let input = vec![
            239, 171, 247, 22, 166, 83, 232, 115, 142, 205, 233, 249, 184, 2, 254, 29,
        ];
        let actual = decrypt_aes256(&input, &key, &salt).unwrap();
        let expected: Vec<u8> = vec![];
        assert_eq!(expected, actual);
    }

    #[test]
    fn aes256_big() {
        let key = StretchKey::mock_stretch(b"1_s3cr3t_p@55w0rd!!");
        let salt = Salt::fixed_for_test(123_456_789_123_456_789);
        let plain = generate_test_file_content_for_test(500_000);
        let input = encrypt_aes256(&plain, &key, &salt);
        assert!(plain != input);
        let actual = decrypt_aes256(&input, &key, &salt).unwrap();
        assert_eq!(plain, actual);
    }

    #[test]
    fn twofish_small() {
        let key = StretchKey::mock_stretch(b"s3cr3t!");
        let salt = Salt::fixed_for_test(123_456_789);
        let input: Vec<u8> = vec![
            116, 245, 144, 10, 177, 86, 56, 253, 69, 146, 58, 191, 153, 12, 201, 127, 91, 29, 0,
            207, 78, 210, 98, 218, 231, 195, 239, 53, 1, 148, 165, 121, 119, 96, 133, 17, 32, 229,
            236, 0, 161, 252, 50, 218, 197, 4, 245, 187, 183, 215, 181, 116, 127, 237, 44, 234,
            123, 17, 87, 102, 163, 3, 224, 95, 109, 189, 86, 58, 72, 213, 63, 79, 171, 77, 194, 58,
            94, 217, 114, 26,
        ];
        let actual = decrypt_twofish(&input, &key, &salt).unwrap();
        let expected = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70,
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn twofish_empty() {
        let key = StretchKey::mock_stretch(b"s3cr3t!");
        let salt = Salt::fixed_for_test(111_555_999);
        let input = vec![
            139, 95, 45, 191, 95, 153, 224, 1, 188, 181, 50, 26, 53, 74, 249, 55,
        ];
        let actual = decrypt_twofish(&input, &key, &salt).unwrap();
        let expected: Vec<u8> = vec![];
        assert_eq!(expected, actual);
    }

    #[test]
    fn twofish_big() {
        let key = StretchKey::mock_stretch(b"1_s3cr3t_p@55w0rd!!");
        let salt = Salt::fixed_for_test(123_456_789_123_456_789);
        let plain = generate_test_file_content_for_test(500_000);
        let input = encrypt_twofish(&plain, &key, &salt);
        assert!(plain != input);
        let actual = decrypt_twofish(&input, &key, &salt).unwrap();
        assert_eq!(plain, actual);
    }
}
