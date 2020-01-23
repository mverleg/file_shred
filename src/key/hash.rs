
//TODO @mark: change to &mut [u8]? does that work with all algorithms?

use ::std::num::NonZeroU32;

use ::ring::pbkdf2::derive;
use ::ring::pbkdf2::PBKDF2_HMAC_SHA256;

use crate::header::KeyHashAlg;

#[inline]
pub fn hash(data: &[u8], salt: &[u8], algorithm: &KeyHashAlg) -> [u8; 32] {
    match algorithm {
        KeyHashAlg::SCrypt => hash_scrypt(data),
        KeyHashAlg::Argon2id => hash_argon2id(data),
        KeyHashAlg::Sha256 => hash_sha256(data, salt),
    }
}

#[inline]
pub fn hash_scrypt(data: &[u8]) -> [u8; 32] {
    unimplemented!()
}

#[inline]
pub fn hash_argon2id(data: &[u8]) -> [u8; 32] {
    unimplemented!()
}

#[inline]
pub fn hash_sha256(data: &[u8], salt: &[u8]) -> [u8; 32] {
    let mut salted = [0; 32];
    derive(PBKDF2_HMAC_SHA256, NonZeroU32::new_unchecked(100), salt, data, &mut salted);
    salted
}
