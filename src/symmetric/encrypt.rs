use ::block_modes::BlockMode;

use crate::header::SymmetricEncryptionAlg;
use crate::key::key::StretchKey;
use crate::key::Salt;

use crate::symmetric::{Aes256Cbc, TwofishCbc};

pub fn encrypt_file(
    mut data: Vec<u8>,
    key: &StretchKey,
    salt: &Salt,
    encrypt_algs: &[SymmetricEncryptionAlg],
) -> Vec<u8> {
    assert!(encrypt_algs.len() >= 1);
    for encrypt_alg in encrypt_algs {
        data = match encrypt_alg {
            SymmetricEncryptionAlg::Aes256 => encrypt_aes256(data, key, salt),
            SymmetricEncryptionAlg::Twofish => encrypt_twofish(data, key, salt),
        }
    }
    data
}

pub fn encrypt_aes256(data: Vec<u8>, key: &StretchKey, salt: &Salt) -> Vec<u8> {
    debug_assert!(key.key_data.unsecure().len() >= 32);
    debug_assert!(salt.salt.len() >= 16);
    let cipher = Aes256Cbc::new_var(&key.key_data.unsecure()[..32], &salt.salt[..16]).unwrap();
    cipher.encrypt_vec(&data)
}

pub fn encrypt_twofish(data: Vec<u8>, key: &StretchKey, salt: &Salt) -> Vec<u8> {
    debug_assert!(key.key_data.unsecure().len() >= 16);
    debug_assert!(salt.salt.len() >= 16);
    let cipher = TwofishCbc::new_var(&key.key_data.unsecure()[..16], &salt.salt[..16]).unwrap();
    cipher.encrypt_vec(&data)
}

#[cfg(test)]
mod tests {

    use ::aes_ctr::stream_cipher::NewStreamCipher;

    use ::twofish::block_cipher_trait::BlockCipher;

    use crate::files::mockfile::generate_test_file_content_for_test;

    use super::*;

    #[test]
    fn aes256_small() {
        let key = StretchKey::mock_stretch("s3cr3t!".as_bytes());
        let salt = Salt::static_for_test(123_456_789);
        let input = vec![
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
            22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
            44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65,
            66, 67, 68, 69, 70,
        ];
        let actual = encrypt_aes256(input, &key, &salt);
        let expected: Vec<u8> = vec![
            8, 161, 111, 221, 11, 228, 30, 113, 127, 148, 186, 160, 217, 32, 132, 212,
            24, 230, 13, 196, 126, 21, 244, 203, 34, 121, 157, 181, 3, 37, 201, 196, 21,
            217, 132, 207, 61, 60, 74, 108, 41, 135, 28, 68, 53, 129, 76, 135, 250, 215,
            77, 32, 13, 150, 85, 84, 172, 159, 42, 60, 228, 3, 21, 221, 83, 195, 0, 15,
            124, 67, 62, 219, 72, 251, 230, 81, 87, 117, 239, 90
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn aes256_empty() {
        let key = StretchKey::mock_stretch("s3cr3t!".as_bytes());
        let salt = Salt::static_for_test(111_555_999);
        let input = vec![];
        let actual = encrypt_aes256(input, &key, &salt);
        let expected: Vec<u8> = vec![239, 171, 247, 22, 166, 83, 232, 115, 142, 205, 233, 249, 184, 2, 254, 29];
        assert_eq!(expected, actual);
    }

    #[test]
    fn aes256_big() {
        let key = StretchKey::mock_stretch("1_s3cr3t_p@55w0rd!!".as_bytes());
        let salt = Salt::static_for_test(123_456_789_123_456_789);
        let input = generate_test_file_content_for_test(500_000);
        let actual = encrypt_aes256(input, &key, &salt);
        let expected_start = &[99, 98, 68, 40, 23, 127, 40, 229];
        let expected_end = &[246, 94, 217, 38, 227, 81, 170, 63];
        assert_eq!(expected_start, &actual[..8]);
        assert_eq!(expected_end, &actual[actual.len()-8..]);
    }

    #[test]
    fn twofish_small() {
        let key = StretchKey::mock_stretch("s3cr3t!".as_bytes());
        let salt = Salt::static_for_test(123_456_789);
        let input = vec![
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
            22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
            44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65,
            66, 67, 68, 69, 70,
        ];
        let actual = encrypt_twofish(input, &key, &salt);
        let expected: Vec<u8> = vec![
            116, 245, 144, 10, 177, 86, 56, 253, 69, 146, 58, 191, 153, 12, 201, 127,
            91, 29, 0, 207, 78, 210, 98, 218, 231, 195, 239, 53, 1, 148, 165, 121,
            119, 96, 133, 17, 32, 229, 236, 0, 161, 252, 50, 218, 197, 4, 245, 187,
            183, 215, 181, 116, 127, 237, 44, 234, 123, 17, 87, 102, 163, 3, 224, 95,
            109, 189, 86, 58, 72, 213, 63, 79, 171, 77, 194, 58, 94, 217, 114, 26
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn twofish_empty() {
        let key = StretchKey::mock_stretch("s3cr3t!".as_bytes());
        let salt = Salt::static_for_test(111_555_999);
        let input = vec![];
        let actual = encrypt_twofish(input, &key, &salt);
        let expected: Vec<u8> = vec![139, 95, 45, 191, 95, 153, 224, 1, 188, 181, 50, 26, 53, 74, 249, 55];
        assert_eq!(expected, actual);
    }

    #[test]
    fn twofish_big() {
        let key = StretchKey::mock_stretch("1_s3cr3t_p@55w0rd!!".as_bytes());
        let salt = Salt::static_for_test(123_456_789_123_456_789);
        let input = generate_test_file_content_for_test(500_000);
        let actual = encrypt_twofish(input, &key, &salt);
        let expected_start = &[123, 234, 159, 158, 79, 48, 128, 175];
        let expected_end = &[64, 227, 233, 211, 40, 252, 244, 86];
        assert_eq!(expected_start, &actual[..8]);
        assert_eq!(expected_end, &actual[actual.len()-8..]);
    }
}
