
use ::block_modes::{BlockMode, Cbc};
use ::block_modes::block_padding::Pkcs7;

use ::aes::Aes128;
use ::twofish::Twofish;
use ::block_modes::block_padding::Iso7816;
use ::aes::Aes256;

type Aes256Cbc = Cbc<Aes256, Iso7816>;
type TwofishCbc = Cbc<Twofish, Iso7816>;

#[test]
fn tmp() {
    let key: &[u8; 32] = b"RvzQW3Mwrc!_y5-DpPZl8rP3,=HsD1,!";
    let iv: &[u8; 16] = b"Zkv6ta7s*JlG7oke";
    let plaintext: Vec<u8> = b"Hello world! This is the secret text...".to_vec();

    let cipher = Aes256Cbc::new_var(key, iv).unwrap();
    let ciphertext = cipher.encrypt_vec(&plaintext);

    assert_eq!(&ciphertext, &vec![
        209, 219, 109, 15, 80, 252, 140, 44, 140, 197, 166, 182, 9, 189, 201, 6,
        182, 184, 170, 77, 128, 173, 101, 165, 175, 18, 176, 10, 108, 228, 48, 102,
        21, 212, 247, 48, 65, 234, 95, 39, 156, 23, 116, 198, 156, 65, 189, 82
    ]);

    let cipher = Aes256Cbc::new_var(key, iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).unwrap();

    assert_eq!(decrypted_ciphertext, plaintext);
}

#[test]
fn find_aes256_size() {

    for key_size in 0..64 {
        for iv_size in 0..64 {
            let key = vec![0u8; key_size];
            let iv = vec![0u8; iv_size];
            match Aes256Cbc::new_var(&key, &iv) {
                Ok(_) => {
                    println!("FOUND! key size {}, iv size {}", key_size, iv_size);
                    assert!(false);
                },
                Err(_) => (),
            }
        }
    }
    assert!(false);
}

#[test]
fn find_twofist_size() {

    for key_size in 0..64 {
        for iv_size in 0..64 {
            let key = vec![0u8; key_size];
            let iv = vec![0u8; iv_size];
            match TwofishCbc::new_var(&key, &iv) {
                Ok(_) => {
                    println!("FOUND! key size {}, iv size {}", key_size, iv_size);
                    assert!(false);
                },
                Err(_) => (),
            }
        }
    }
    assert!(false);
}
