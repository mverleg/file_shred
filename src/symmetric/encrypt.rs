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

pub fn encrypt_aes256(mut data: Vec<u8>, key: &StretchKey, salt: &Salt) -> FedResult<Vec<u8>> {
    //let key_ga = GenericArray::clone_from_slice(&key.key_data.unsecure());
    //let cipher = Aes256::new(&key_ga);
    //cipher.encrypt_block(&data);

    println!("C1") ; //TODO @mark: TEMPORARY! REMOVE THIS!
    dbg!(&key.key_data.unsecure());  //TODO @mark: TEMPORARY! REMOVE THIS!
    //let key_data = ;
    println!("C2") ; //TODO @mark: TEMPORARY! REMOVE THIS!
    let key = GenericArray::from_slice(key.key_data.unsecure());
    println!("D") ; //TODO @mark: TEMPORARY! REMOVE THIS!
    let nonce = GenericArray::from_slice(&salt.salt);
    println!("E") ; //TODO @mark: TEMPORARY! REMOVE THIS!
    let mut cipher = Aes256Ctr::new(&key, &nonce);
    println!("F") ; //TODO @mark: TEMPORARY! REMOVE THIS!
    cipher.apply_keystream(&mut data);
    println!("G") ; //TODO @mark: TEMPORARY! REMOVE THIS!
    //assert_eq!(data, [6, 245, 126, 124, 180, 146, 37]);

    unimplemented!()
}

pub fn encrypt_blowfish(mut data: Vec<u8>, key: &StretchKey) -> FedResult<Vec<u8>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aes_ctr::Aes128Ctr;

    #[test]
    fn aes_ctr_sanity_check_demo() {
        let mut data = [1, 2, 3, 4, 5, 6, 7];
        let key = GenericArray::from_slice(b"very secret key.");
        let nonce = GenericArray::from_slice(b"and secret nonce");
        let mut cipher = Aes128Ctr::new(&key, &nonce);
        cipher.apply_keystream(&mut data);
        assert_eq!(data, [6, 245, 126, 124, 180, 146, 37]);
        cipher.seek(0);
        cipher.apply_keystream(&mut data);
        assert_eq!(data, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn aes_ctr_sanity_check_own() {
        let salt = Salt::static_for_test(123_456_789);
        let mut input = vec![
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
            64, 65, 66, 67, 68, 69, 70,
        ];
        let key = GenericArray::from_slice("s3cr3t!".as_bytes());
        let nonce = GenericArray::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]);
        let mut cipher = Aes256Ctr::new(&key, &nonce);
        cipher.apply_keystream(&mut input);
    }

    #[test]
    fn aes256_small() {
        println!("A") ; //TODO @mark: TEMPORARY! REMOVE THIS!
        let key = StretchKey::new("s3cr3t!".as_bytes());
        let salt = Salt::static_for_test(123_456_789);
        let input = vec![
            00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
            64, 65, 66, 67, 68, 69, 70,
        ];
        println!("B") ; //TODO @mark: TEMPORARY! REMOVE THIS!
        let actual = encrypt_aes256(input, &key, &salt).unwrap();
        dbg!(&actual);  //TODO @mark: TEMPORARY! REMOVE THIS!
        let expected: Vec<u8> = vec![];
        assert_eq!(actual, expected);
    }

    //TODO @mark: more tests
}
