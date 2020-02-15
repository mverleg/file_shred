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
    // version of the input key, so isn't limited to printable ascii.
    let key = SecVec::from(b"RvzQW3Mwrc!_y5-DpPZl8rP3,=HsD1,!");

    // The initialization vector (like salt or nonce) must be 16 bytes for
    // this mode. It could be generated using a secure randon generator.
    let iv = SecVec::from(vec![
        89, 63, 254, 34, 209, 155, 236, 158, 195, 104, 11, 16, 240, 4, 26, 76
    ]);

    // This is the data that is to be encrypted.
    let plaintext: Vec<u8> = b"Hello world! This is the secret text...".to_vec();

    // Encryption.
    let cipher = Aes256Cbc::new_var(key.unsecure(), iv.unsecure()).unwrap();
    let ciphertext = cipher.encrypt_vec(&plaintext);

    // Check that it worked.
    assert_eq!(&ciphertext, &vec![
        209, 219, 109, 15, 80, 252, 140, 44, 140, 197, 166, 182, 9, 189, 201, 6,
        182, 184, 170, 77, 128, 173, 101, 165, 175, 18, 176, 10, 108, 228, 48, 102,
        21, 212, 247, 48, 65, 234, 95, 39, 156, 23, 116, 198, 156, 65, 189, 82
    ]);

    // Decryption.
    let cipher = Aes256Cbc::new_var(key.unsecure(), iv.unsecure()).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).unwrap();

    // Check that the original input is returned.
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
