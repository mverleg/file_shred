use ::std::fs;

use crate::config::enc::EncryptConfig;
use crate::files::checksum::calculate_checksum;
use crate::files::compress::compress_file;
use crate::files::file_meta::inspect_files;
use crate::header::strategy::get_current_version_strategy;
use crate::key::stretch::stretch_key;
use crate::key::Salt;
use crate::symmetric::encrypt::encrypt_file;
use crate::util::util::wrap_io;
use crate::util::FedResult;

pub mod config;
pub mod files;
pub mod header;
pub mod key;
pub mod symmetric;
pub mod util;

pub fn encrypt(config: &EncryptConfig) -> FedResult<()> {
    let strategy = get_current_version_strategy(config.debug());
    let files_info = inspect_files(config.files(), config.debug())?;
    let _total_size_kb: u64 = files_info.iter().map(|inf| inf.size_kb).sum();
    let salt = Salt::generate_random()?;
    let stretched_key = stretch_key(
        config.raw_key(),
        &salt,
        strategy.stretch_count,
        &strategy.key_hash_algorithms,
    );
    //TODO @mark: progress logging
    for file in files_info {
        if config.debug() {
            println!("encrypting {}", file.path_str());
        }
        if file.size_kb > 1024 * 1024 {
            eprintln!(
                "warning: reading {} Mb file {} into RAM",
                file.size_kb / 1024,
                file.path_str()
            );
        }
        let data = wrap_io(fs::read(file.path))?;
        let _checksum = calculate_checksum(&data);
        let small = compress_file(data, &strategy.compression_algorithm)?;
        let _secret = encrypt_file(small, &stretched_key, &salt, &strategy.symmetric_algorithms);
    }
    unimplemented!()
}

pub fn decrypt(_config: &EncryptConfig) -> FedResult<()> {
    unimplemented!() //TODO @mark:
}

/// The demo used in this blog post:
/// https://markv.nl/blog/symmetric-encryption-in-rust
#[cfg(test)]
mod tests {
    use ::aes::Aes256;
    use ::block_modes::block_padding::Iso7816;
    use ::block_modes::BlockMode;
    use ::block_modes::Cbc;
    use ::secstr::SecVec;

    type Aes256Cbc = Cbc<Aes256, Iso7816>;

    /// Open the files in 'test_files/' that were encrypted with previous versions,
    /// and make sure they can still be decrypted (and match the original).
    #[test]
    fn compatibility() {

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
