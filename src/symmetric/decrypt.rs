use crate::header::SymmetricEncryptionAlg;
use crate::key::key::StretchKey;
use crate::key::Salt;
use crate::symmetric::shared::endec_aes256;
use crate::util::FedResult;
use crate::symmetric::{TwofishCbc, Aes256Cbc};
use block_modes::BlockMode;

pub fn decrypt_file(
    mut data: Vec<u8>,
    key: &StretchKey,
    salt: &Salt,
    encrypt_algs: &[SymmetricEncryptionAlg],
) -> FedResult<Vec<u8>> {
    assert!(encrypt_algs.len() >= 1);
    for encrypt_alg in encrypt_algs {
        data = match encrypt_alg {
            SymmetricEncryptionAlg::Aes256 => decrypt_aes256(data, key, salt)?,
            SymmetricEncryptionAlg::Twofish => decrypt_twofish(data, key, salt)?,
        }
    }
    Ok(data)
}

pub fn decrypt_aes256(data: Vec<u8>, key: &StretchKey, salt: &Salt) -> FedResult<Vec<u8>> {
    debug_assert!(key.key_data.unsecure().len() >= 32);
    debug_assert!(salt.salt.len() >= 16);
    let cipher = Aes256Cbc::new_var(
        &key.key_data.unsecure()[..32],
        &salt.salt[..16]
    ).unwrap();
    match cipher.decrypt_vec(&data) {
        Ok(plain) => Ok(plain),
        Err(err) => Err(format!("Decryption algorithm failed: {}", err))
    }
}

pub fn decrypt_twofish(data: Vec<u8>, key: &StretchKey, salt: &Salt) -> FedResult<Vec<u8>> {
    debug_assert!(key.key_data.unsecure().len() >= 16);
    debug_assert!(salt.salt.len() >= 16);
    let cipher = TwofishCbc::new_var(
        &key.key_data.unsecure()[..16],
        &salt.salt[..16]
    ).unwrap();
    match cipher.decrypt_vec(&data) {
        Ok(plain) => Ok(plain),
        Err(err) => Err(format!("Decryption algorithm failed: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use ::aes_ctr::stream_cipher::generic_array::GenericArray;
    use ::aes_ctr::stream_cipher::NewStreamCipher;
    use ::aes_ctr::stream_cipher::SyncStreamCipher;
    use ::aes_ctr::Aes256Ctr;

    use crate::files::mockfile::generate_test_file_content_for_test;
    use crate::key::hash::fastish_hash;
    use crate::symmetric::encrypt::encrypt_aes256;

    use super::*;

    #[test]
    fn aes256_small() {
        let key = StretchKey::mock_stretch("s3cr3t!".as_bytes());
        let salt = Salt::static_for_test(123_456_789);
        let input: Vec<u8> = vec![
            17, 154, 31, 230, 44, 192, 227, 243, 227, 255, 149, 126, 201, 220, 251, 132, 219, 73,
            148, 126, 63, 78, 9, 183, 92, 130, 232, 146, 102, 241, 230, 77, 71, 87, 16, 59, 74,
            232, 36, 39, 65, 134, 137, 244, 105, 165, 16, 151, 233, 85, 60, 40, 168, 116, 45, 121,
            23, 224, 34, 33, 78, 203, 137, 199, 157, 211, 173, 61, 101, 153, 139,
        ];
        let actual = decrypt_aes256(input, &key, &salt).unwrap();
        let expected = vec![
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
            22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
            44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65,
            66, 67, 68, 69, 70,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn aes256_big() {
        let key = StretchKey::mock_stretch("1_s3cr3t_p@55w0rd!!".as_bytes());
        let salt = Salt::static_for_test(123_456_789_123_456_789);
        let input = generate_test_file_content_for_test(1_000_000);
        let actual = decrypt_aes256(input, &key, &salt).unwrap();
        let expected_start = &[81, 163, 93, 212, 203, 139, 62, 17];
        assert_eq!(&actual[..8], expected_start);
    }
}
