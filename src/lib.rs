use ::std::fs;

use crate::config::enc::EncryptConfig;
use crate::files::compress::compress_file;
use crate::files::file_meta::inspect_files;
use crate::header::strategy::get_current_version_strategy;
use crate::key::Salt;
use crate::key::stretch::stretch_key;
use crate::symmetric::encrypt::encrypt_file;
use crate::util::FedResult;
use crate::util::util::wrap_io;

pub mod util;
pub mod header;
pub mod key;
pub mod symmetric;
pub mod config;
pub mod files;

pub fn encrypt(config: &EncryptConfig) -> FedResult<()> {
    let strategy = get_current_version_strategy(config.debug());
    let files_info = inspect_files(config.files(), config.debug())?;
    let total_size_kb: u64 = files_info.iter().map(|inf| inf.size_kb).sum();
    let salt = Salt::generate_random()?;
    let stretched_key = stretch_key(config.raw_key(), &salt, strategy.stretch_count, &strategy.key_hash_algorithms);
    //TODO @mark: progress logging
    for file in files_info {
        if config.debug() {
            println!("encrypting {}", file.path_str());
        }
        if file.size_kb > 1024*1024 {
            eprintln!("warning: reading {} Mb file {} into RAM", file.size_kb / 1024, file.path_str());
        }
        let mut data = wrap_io(fs::read(file.path))?;
        data = compress_file(data, &strategy.compression_algorithm)?;
        data = encrypt_file(data, &stretched_key, &salt, &strategy.symmetric_algorithms)?;
    }
    unimplemented!()
}

pub fn decrypt(_config: &EncryptConfig) -> FedResult<()> {
    unimplemented!()  //TODO @mark:
}
