use crate::config::enc::EncryptConfig;
use crate::files::file_meta::inspect_files;
use crate::header::strategy::get_current_version_strategy;
use crate::key::Salt;
use crate::key::stretch::stretch_key;
use crate::util::FedResult;

pub mod util;
pub mod header;
pub mod key;
pub mod config;
pub mod files;

pub fn encrypt(config: &EncryptConfig) -> FedResult<()> {
    let strategy = get_current_version_strategy(config.debug());
    let files_info = inspect_files(config.files(), config.debug())?;
    let total_size_kb: u64 = files_info.iter().map(|inf| inf.size_kb).sum();
    let salt = Salt::generate_random();
    //TODO @mark: progress logging
    for file in files_info {
        let stretched_key = stretch_key(config.raw_key(), &salt, strategy.stretch_count, &strategy.key_hash_algorithms);

    }
    unimplemented!()
}

pub fn decrypt(_config: &EncryptConfig) -> FedResult<()> {
    unimplemented!()  //TODO @mark:
}
