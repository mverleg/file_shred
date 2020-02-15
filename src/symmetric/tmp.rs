use ::aes::Aes128;
use ::aes::Aes256;
use ::block_modes::{BlockMode, Cbc};
use ::block_modes::block_padding::Iso7816;
use ::block_modes::block_padding::Pkcs7;
use ::secstr::SecVec;
use ::twofish::Twofish;
use ::ring::pbkdf2::derive;
use ring::pbkdf2::PBKDF2_HMAC_SHA512;

// Set up the cipher.
// Cbc means each block affects the next, which is more secure than Ecb.
// Aes256 is the actual encryption/decryption algorithm.
// Iso7816 is the padding using if the data doesn't fit in the block size.
type Aes256Cbc = Cbc<Aes256, Iso7816>;
type TwofishCbc = Cbc<Twofish, Iso7816>;

#[test]
fn demo() {
    // Key must be 32 bytes for Aes256. It should probably be the hashed
    // version of the input key, so is not limited to printable ascii.
    let key = SecVec::from(b"RvzQW3Mwrc!_y5-DpPZl8rP3,=HsD1,!".to_vec());

    // The initialization vector (like salt or nonce) must be 16 bytes for
    // this block size. It could be generated using a secure randon generator,
    // and should be different each time. It is not a secret.
    let iv = SecVec::from(vec![
        89, 63, 254, 34, 209, 155, 236, 158, 195, 104, 11, 16, 240, 4, 26, 76
    ]);

    // This is the data that is to be encrypted.
    let plaintext: Vec<u8> = b"Hello world! This is the secret text...".to_vec();

    // Encryption.
    let cipher = Aes256Cbc::new_var(
        key.unsecure(), iv.unsecure()).unwrap();
    let ciphertext = cipher.encrypt_vec(&plaintext);

    // Check that it worked.
    assert_eq!(&ciphertext, &vec![
        216, 56, 166, 254, 171, 163, 243, 167, 235, 179, 189, 132, 0, 202, 44, 73,
        10, 68, 229, 90, 69, 212, 24, 22, 87, 109, 34, 92, 254, 136, 141, 154, 57,
        189, 176, 221, 140, 8, 114, 141, 103, 248, 108, 182, 247, 156, 113, 127,
    ]);

    // Decryption.
    let cipher = Aes256Cbc::new_var(
        key.unsecure(), iv.unsecure()).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).unwrap();

    // Check that we got the original input back.
    assert_eq!(decrypted_ciphertext, plaintext);
}

//#[test]
//fn find_aes256_size() {
//
//    for key_size in 0..64 {
//        for iv_size in 0..64 {
//            let key = vec![0u8; key_size];
//            let iv = vec![0u8; iv_size];
//            match Aes256Cbc::new_var(&key, &iv) {
//                Ok(_) => {
//                    println!("FOUND! key size {}, iv size {}", key_size, iv_size);
//                    assert!(false);
//                },
//                Err(_) => (),
//            }
//        }
//    }
//    assert!(false);
//}
//
//#[test]
//fn find_twofist_size() {
//
//    for key_size in 0..64 {
//        for iv_size in 0..64 {
//            let key = vec![0u8; key_size];
//            let iv = vec![0u8; iv_size];
//            match TwofishCbc::new_var(&key, &iv) {
//                Ok(_) => {
//                    println!("FOUND! key size {}, iv size {}", key_size, iv_size);
//                    assert!(false);
//                },
//                Err(_) => (),
//            }
//        }
//    }
//    assert!(false);
//}
