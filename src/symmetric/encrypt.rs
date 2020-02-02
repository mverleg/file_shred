use crate::key::key::StretchKey;
use crate::header::SymmetricEncryptionAlg;
use crate::util::FedResult;

pub fn encrypt_file(file: &[u8], key: &StretchKey, algs: &[SymmetricEncryptionAlg]) -> FedResult<Vec<u8>> {


    unimplemented!()
}
