use ::std::fs;

use crate::config::enc::EncryptConfig;
use crate::config::typ::EndecConfig;
use crate::config::DecryptConfig;
use crate::files::checksum::calculate_checksum;
use crate::files::compress::{compress_file, decompress_file};
use crate::files::file_meta::inspect_files;
use crate::files::write_output::write_output_file;
use crate::header::strategy::get_current_version_strategy;
use crate::header::Header;
use crate::header::HEADER_MARKER;
use crate::key::stretch::stretch_key;
use crate::key::Salt;
use crate::symmetric::encrypt::encrypt_file;
use crate::util::errors::wrap_io;
use crate::util::version::get_current_version;
use crate::util::FedResult;
use crate::orchestrate::common_steps::read_file;
use crate::symmetric::decrypt::decrypt_file;

pub fn decrypt(config: &DecryptConfig) -> FedResult<()> {
    if config.delete_input() {
        unimplemented!("deleting input not implemented"); //TODO @mark
    }
    let version = get_current_version();
    let strategy = get_current_version_strategy(config.debug());
    let files_info = inspect_files(unimplemented!()/*config*/)?;
    let _total_size_kb: u64 = files_info.iter().map(|inf| inf.size_kb).sum();
    let salt = Salt::generate_random()?;
    let stretched_key = stretch_key(
        config.raw_key(),
        &salt,
        strategy.stretch_count,
        &strategy.key_hash_algorithms,
    );
    //TODO @mark: progress logging
    for file in &files_info {
        let data = read_file(file, &config.verbosity())?;
        let checksum = calculate_checksum(&data);
        let small = decompress_file(data, &strategy.compression_algorithm)?;
        let secret = decrypt_file(small, &stretched_key, &salt, &strategy.symmetric_algorithms)?;
        let header = Header::new(version.clone(), salt.clone(), checksum, config.debug())?;
        if !config.quiet() {
            println!(
                "successfully encrypted '{}' ({} kb); not saving to '{}' because of dry-run",
                file.path_str(),
                secret.len() / 1024,
                &file.out_pth.to_string_lossy()
            );
        }
    }
    if !config.quiet() {
        println!("encrypted {} files", files_info.len());
    }
    Ok(())
}

/// The demo used in this blog post:
/// https://markv.nl/blog/symmetric-encryption-in-rust
#[cfg(test)]
mod tests {
    use ::std::fs::File;
    use ::std::io::Read;
    use std::env::temp_dir;
    use std::fs;

    use ::aes::Aes256;
    use ::block_modes::block_padding::Iso7816;
    use ::block_modes::BlockMode;
    use ::block_modes::Cbc;
    use ::lazy_static::lazy_static;
    use ::regex::Regex;
    use ::secstr::SecVec;
    use ::semver::Version;

    use crate::config::{DecryptConfig, EncryptConfig};
    use crate::files::scan::get_enc_files_direct;
    use crate::files::scan::TEST_FILE_DIR;
    use crate::header::strategy::Verbosity;
    use crate::key::key::Key;
    use crate::util::version::get_current_version;
    use crate::{decrypt, encrypt};

    type Aes256Cbc = Cbc<Aes256, Iso7816>;

    lazy_static! {
        static ref COMPAT_KEY: Key = Key::new(" LP0y#shbogtwhGjM=*jFFZPmNd&qBO+ ");
        static ref COMPAT_FILE_RE: Regex = Regex::new(r"^original_v(\d+\.\d+\.\d+).png$").unwrap();
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
            .map(|v| Version::parse(&v[1]).unwrap())
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
                Verbosity::Debug,
                false,
                false,
                None,
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
}
