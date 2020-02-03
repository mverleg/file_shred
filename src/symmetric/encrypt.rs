use ::crypto::aes;
use ::crypto::blockmodes;
use ::crypto::buffer;
use ::crypto::symmetriccipher;

use crate::header::SymmetricEncryptionAlg;
use crate::key::key::StretchKey;
use crate::key::Salt;
use crate::util::FedResult;

pub fn encrypt_file(mut data: Vec<u8>, key: &StretchKey, salt: &Salt, encrypt_algs: &[SymmetricEncryptionAlg]) -> FedResult<Vec<u8>> {
    assert!(encrypt_algs.len() >= 1);
    for encrypt_alg in encrypt_algs {
        data = match encrypt_alg {
            SymmetricEncryptionAlg::Aes256 => encrypt_aes256(data, key, salt)?,
            SymmetricEncryptionAlg::Blowfish => encrypt_blowfish(data, key)?,
        }
    }
    Ok(data)
}

pub fn encrypt_aes256(mut data: Vec<u8>, key: &StretchKey, salt: &Salt) -> FedResult<Vec<u8>> {
    let encryptor: Box<dyn symmetriccipher::Encryptor> = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key.key_data.unsecure(),
        &salt.salt,
        blockmodes::PkcsPadding
    );
    let mut read_buffer = buffer::RefReadBuffer::new(&data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
}

pub fn encrypt_blowfish(mut data: Vec<u8>, key: &StretchKey) -> FedResult<Vec<u8>> {
    unimplemented!()
}

//TODO @mark: test

