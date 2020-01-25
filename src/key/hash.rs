
//TODO @mark: change to &mut [u8]? does that work with all algorithms?

use ::std::num::NonZeroU32;

use ::ring::pbkdf2::derive;
use ::ring::pbkdf2::PBKDF2_HMAC_SHA256;

use crate::header::KeyHashAlg;

#[inline]
pub fn hash(data: &[u8], salt: &[u8], algorithm: &KeyHashAlg) -> Vec<u8> {
    match algorithm {
        KeyHashAlg::SCrypt => hash_scrypt(data, salt),
        KeyHashAlg::Argon2id => hash_argon2id(data, salt),
        KeyHashAlg::Sha256 => hash_sha256(data, salt),
    }
}

#[inline]
pub fn hash_scrypt(data: &[u8], salt: &[u8]) -> Vec<u8> {
    unimplemented!()
}

#[inline]
pub fn hash_argon2id(data: &[u8], salt: &[u8]) -> Vec<u8> {
    unimplemented!()
}

#[inline]
pub fn hash_sha256(data: &[u8], salt: &[u8]) -> Vec<u8> {
    let mut output = vec![0; 32];
    derive(PBKDF2_HMAC_SHA256, NonZeroU32::new(3000).unwrap(), salt, data, &mut output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_scrypt() {
        hash_scrypt(&vec![1; 32], &vec![2; 32]);
    }

    #[test]
    fn test_hash_argon2id() {
        hash_argon2id(&vec![1; 32], &vec![2; 32]);
    }

    #[test]
    fn test_hash_sha256() {
        hash_sha256(&vec![1; 32], &vec![2; 32]);
    }
}
