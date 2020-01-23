
//TODO @mark: change to &mut [u8]? does that work with all algorithms?

use crate::header::KeyHashAlg;

#[inline]
pub fn hash(data: &mut Vec<u8>, algorithm: &KeyHashAlg) {
    match algorithm {
        KeyHashAlg::SCrypt => hash_scrypt(data),
        KeyHashAlg::Argon2id => hash_argon2id(data),
        KeyHashAlg::Sha256 => hash_sha256(data),
    }
}

#[inline]
pub fn hash_scrypt(data: &mut [u8]) {
    unimplemented!()
}

#[inline]
pub fn hash_argon2id(data: &mut [u8]) {
    unimplemented!()
}

#[inline]
pub fn hash_sha256(data: &mut [u8]) {
    unimplemented!()
}
