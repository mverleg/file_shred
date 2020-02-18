use ::std::fs;

use crate::config::enc::EncryptConfig;
use crate::config::typ::EndecConfig;
use crate::config::DecryptConfig;
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

pub fn decrypt(_config: &DecryptConfig) -> FedResult<()> {
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
    use ::lazy_static::lazy_static;
    use ::regex::Regex;
    use ::secstr::SecVec;
    use ::semver::Version;
    use ::std::fs::File;
    use ::std::io::Read;

    use crate::config::{DecryptConfig, EncryptConfig};
    use crate::decrypt;
    use crate::files::scan::get_enc_files_direct;
    use crate::files::scan::TEST_FILE_DIR;
    use crate::key::key::Key;
    use crate::util::version::get_current_version;
    use std::env::temp_dir;

    type Aes256Cbc = Cbc<Aes256, Iso7816>;

    lazy_static! {
        static ref COMPAT_KEY: Key = Key::new(" LP0y#shbogtwhGjM=*jFFZPmNd&qBO+ ");
        static ref COMPAT_FILE_RE: Regex = Regex::new(r"^original_v(\d+\.\d+\.\d+).png$").unwrap();
    }

    #[test]
    fn store_current_version() {
        let version = get_current_version();
        let mut in_pth = TEST_FILE_DIR.clone();
        in_pth.push("original.png");
        assert!(in_pth.exists());
        println!("{} -> {}", &in_pth.to_string_lossy(), &out_pth.to_string_lossy());
        let conf = EncryptConfig::new(
            vec![in_pth.to_str().unwrap()],
            COMPAT_KEY.clone(),
            false,  // debug
            true,  // overwrite
            false,  // delete_input
            Some(temp_dir()),  // output_dir
            ".enc".to_string(),  // output_extension
            false,  //dry_run
        );
        let mut out_pth = TEST_FILE_DIR.clone();
        out_pth.push(format!("original_v{}.png.enc", version));
        
        unimplemented!()
    }

    /// Open the files in 'test_files/' that were encrypted with previous versions,
    /// and make sure they can still be decrypted (and match the original).
    #[test]
    fn load_all_versions() {
        let enc_files: Vec<Version> = get_enc_files_direct(&*TEST_FILE_DIR)
            .unwrap()
            .iter()
            .map(|f| f.file_stem().unwrap().to_str().unwrap())
            .map(|n| COMPAT_FILE_RE.captures_iter(n).next().unwrap())
            .map(|v| Version::parse(&v[0]).unwrap())
            .collect();
        assert!(!enc_files.is_empty());
        let mut original_pth = TEST_FILE_DIR.clone();
        original_pth.push("original.png".to_owned());
        for enc_file in enc_files {
            let mut enc_pth = TEST_FILE_DIR.clone();
            enc_pth.push(format!("original_v{}.png.enc", enc_file));
            let mut dec_pth = TEST_FILE_DIR.clone();
            dec_pth.push(format!("original_v{}.png", enc_file));
            let conf = DecryptConfig::new(
                vec![enc_pth],
                COMPAT_KEY.clone(),
                false, // debug
                false, // overwrite
                false, // delete_input
                None,  // output_dir
                ".enc".to_owned(),
            );
            decrypt(&conf).unwrap();
            let mut original_data = vec![];
            File::open(&original_pth)
                .unwrap()
                .read_to_end(&mut original_data)
                .unwrap();
            let mut dec_data = vec![];
            File::open(&dec_pth)
                .unwrap()
                .read_to_end(&mut dec_data)
                .unwrap();
            unimplemented!() //TODO @mark: check that the file written is the same as original
        }
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
