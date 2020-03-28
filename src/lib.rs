
pub use crate::orchestrate::decrypt::decrypt;
// These two methods are the main entry points.
pub use crate::orchestrate::encrypt::encrypt;

pub mod util;

/// The demo used in this blog post:
/// https://markv.nl/blog/symmetric-encryption-in-rust
#[cfg(test)]
mod tests {
    use ::aes::Aes256;
    use ::block_modes::block_padding::Iso7816;
    use ::block_modes::BlockMode;
    use ::block_modes::Cbc;
    use ::lazy_static::lazy_static;
    use ::regex::Regex;
    use ::secstr::SecVec;

    use crate::key::key::Key;

    type Aes256Cbc = Cbc<Aes256, Iso7816>;

    lazy_static! {
        static ref COMPAT_KEY: Key = Key::new(" LP0y#shbogtwhGjM=*jFFZPmNd&qBO+ ");
        static ref COMPAT_FILE_RE: Regex = Regex::new(r"^original_v(\d+\.\d+\.\d+).png$").unwrap();
    }

    #[test]
    fn demo() {
        // Key must be 32 bytes for Aes256. It should probably be the hashed
        // version of the input key, so is not limited to printable ascii.
        // SecVec has several advantages in preventing the key from leaking.
        let key = SecVec::from(b"RvzQW3Mwrc!_y5-DpPZl8rP3,=HsD1,!".to_vec());

        // The initialization vector (like salt or nonce) must be 16 bytes for
        // this block size. It could be generated using a secure randon generator,
        // and should be different each time. It is not a secret.
        let iv = vec![
            89, 63, 254, 34, 209, 155, 236, 158, 195, 104, 11, 16, 240, 4, 26, 76,
        ];

        // This is the data that is to be encrypted.
        let plaintext: Vec<u8> = b"Hello world! This is the secret text...".to_vec();

        // Encryption.
        // Fails if the key or iv are the wrong length, so it is safe to unwrap
        // as we have the correct lengths. Key length depends on algorithm, iv length
        // depends on the block size. If it's not documented, experiment with 16 or 32.
        let cipher = Aes256Cbc::new_var(key.unsecure(), &iv).unwrap();
        let ciphertext = cipher.encrypt_vec(&plaintext);

        // Check that it worked.
        assert_eq!(
            &ciphertext,
            &vec![
                216, 56, 166, 254, 171, 163, 243, 167, 235, 179, 189, 132, 0, 202, 44, 73, 10, 68,
                229, 90, 69, 212, 24, 22, 87, 109, 34, 92, 254, 136, 141, 154, 57, 189, 176, 221,
                140, 8, 114, 141, 103, 248, 108, 182, 247, 156, 113, 127,
            ]
        );

        // Decryption.
        let cipher = Aes256Cbc::new_var(key.unsecure(), &iv).unwrap();
        let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).unwrap();

        // Check that we got the original input back.
        assert_eq!(decrypted_ciphertext, plaintext);
    }
}
