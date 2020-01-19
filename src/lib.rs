pub mod util;
pub mod header;

use crate::util::FedResult;

#[derive(Debug)]
pub struct EncryptConfig {

}

#[derive(Debug)]
pub struct DecryptConfig {

}

pub fn encrypt(config: &EncryptConfig) -> FedResult<()> {
    unimplemented!()  //TODO @mark:
}

pub fn decrypt(config: &EncryptConfig) -> FedResult<()> {
    unimplemented!()  //TODO @mark:
}
