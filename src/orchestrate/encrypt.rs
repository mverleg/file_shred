use ::std::fs;

use crate::config::enc::EncryptConfig;
use crate::config::typ::EndecConfig;
use crate::config::DecryptConfig;
use crate::files::checksum::calculate_checksum;
use crate::files::compress::compress_file;
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

pub fn encrypt(config: &EncryptConfig) -> FedResult<()> {
    if config.delete_input() {
        unimplemented!("deleting input not implemented"); //TODO @mark
    }
    let version = get_current_version();
    let strategy = get_current_version_strategy(config.debug());
    let files_info = inspect_files(config)?;
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
        let small = compress_file(data, &strategy.compression_algorithm)?;
        let secret = encrypt_file(small, &stretched_key, &salt, &strategy.symmetric_algorithms);
        let header = Header::new(version.clone(), salt.clone(), checksum, config.debug())?;
        if !config.dry_run() {
            write_output_file(config, &file, &secret, &header)?;
        } else if !config.quiet() {
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

    #[test]
    fn store_current_version() {
        let version = get_current_version();
        let in_pth = {
            let mut p = TEST_FILE_DIR.clone();
            p.push("original.png");
            p
        };
        assert!(in_pth.exists());
        let conf = EncryptConfig::new(
            vec![in_pth],
            COMPAT_KEY.clone(),
            Verbosity::Debug,
            true,
            false,
            Some(temp_dir()),
            ".enc".to_string(),
            false,
        );
        let tmp_pth = {
            let mut p = temp_dir();
            p.push("original.png.enc");
            p
        };
        encrypt(&conf).unwrap();
        assert!(tmp_pth.is_file(), "encrypted file was not created");
        let store_pth = {
            let mut p = TEST_FILE_DIR.clone();
            p.push(format!("original_v{}.png.enc", version));
            p
        };
        if !store_pth.exists() {
            println!("storing file for new version {} as part of backward compatibility test files:\n{} -> {}",
                     version, &tmp_pth.to_string_lossy(), &store_pth.to_string_lossy());
            fs::copy(&tmp_pth, &store_pth).unwrap();
        }
        // Remove the temporary file (as a courtesy, not critical).
        println!(
            "removing temporary file {} for version {}",
            &tmp_pth.to_string_lossy(),
            version
        );
        fs::remove_file(tmp_pth).unwrap();
    }
}
