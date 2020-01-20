use crate::util::FedResult;
use crate::config::enc::EncryptConfig;

pub mod util;
pub mod header;
pub mod key;
pub mod config;

pub fn encrypt(config: &EncryptConfig) -> FedResult<()> {

    unimplemented!()  //TODO @mark:
}

pub fn decrypt(_config: &EncryptConfig) -> FedResult<()> {
    unimplemented!()  //TODO @mark:
}
