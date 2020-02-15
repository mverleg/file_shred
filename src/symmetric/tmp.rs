
use ::block_modes::{BlockMode, Cbc};
use ::block_modes::block_padding::Pkcs7;

use aes::Aes128;
use twofish::Twofish;

// create an alias for convenience
type Aes128Cbc = Cbc<Aes128, Pkcs7>;
type TwofishCbc = Cbc<Twofish, Pkcs7>;

#[allow(unused_imports)]
fn tmp() {
    let key = b"000102030405060708090a0b0c0d0e0f";
    let iv = b"f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff";
    let plaintext = b"Hello world!";
    let cipher = Aes128Cbc::new_var(key, iv).unwrap();

    // buffer must have enough space for message+padding
    let mut buffer = [0u8; 32];
    // copy message to the buffer
    let pos = plaintext.len();
    buffer[..pos].copy_from_slice(plaintext);
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

    assert_eq!(ciphertext, b"1b7a4c403124ae2fb52bedc534d82fa8");

    // re-create cipher mode instance and decrypt the message
    let cipher = Aes128Cbc::new_var(key, iv).unwrap();
    let mut buf = ciphertext.to_vec();
    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();

    assert_eq!(decrypted_ciphertext, plaintext);

    let cipher = Aes128Cbc::new_var(key, iv).unwrap();
    let ciphertext = cipher.encrypt_vec(plaintext);

    assert_eq!(ciphertext, b"1b7a4c403124ae2fb52bedc534d82fa8");

    let cipher = Aes128Cbc::new_var(key, iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).unwrap();

    assert_eq!(decrypted_ciphertext, plaintext);
}
