
use ::std::num::NonZeroU32;

use ::argon2rs::Argon2;
use ::argon2rs::Variant;
use ::crypto::scrypt::scrypt;
use ::crypto::scrypt::ScryptParams;
use ::lazy_static::lazy_static;
use ::ring::pbkdf2::derive;
use ::ring::pbkdf2::PBKDF2_HMAC_SHA512;

use crate::header::KeyHashAlg;

#[inline]
pub fn hash(data: &[u8], salt: &[u8], algorithm: &KeyHashAlg) -> Vec<u8> {
    match algorithm {
        KeyHashAlg::SCrypt => hash_scrypt(data, salt),
        KeyHashAlg::Argon2i => hash_argon2i(data, salt),
        KeyHashAlg::Sha512 => hash_sha256(data, salt),
    }
}

lazy_static! {
    // NOTE: if the hash parameters ever change, then KeyHashAlg must be extended to reflect that,
    // to make sure that anything encrypted before can still be decrypted.
    // https://crypto.stackexchange.com/questions/35423
    static ref SCRYPT_PARAMS: ScryptParams = ScryptParams::new(14, 8, 1);
}

lazy_static! {
    // NOTE: if the hash parameters ever change, then KeyHashAlg must be extended to reflect that,
    // to make sure that anything encrypted before can still be decrypted.
    static ref ARGON_CONFIG: Argon2 = Argon2::new(30, 8, 4096, Variant::Argon2i).unwrap();
}

lazy_static! {
    // NOTE: if the hash parameters ever change, then KeyHashAlg must be extended to reflect that,
    // to make sure that anything encrypted before can still be decrypted.
    static ref SHA_REPS: NonZeroU32 = NonZeroU32::new(70_000).unwrap();
}

#[inline]
pub fn hash_scrypt(data: &[u8], salt: &[u8]) -> Vec<u8> {
    let mut output = vec![0; 32];
    scrypt(salt, data, &*SCRYPT_PARAMS, &mut output);
    output
}

#[inline]
pub fn hash_argon2i(data: &[u8], salt: &[u8]) -> Vec<u8> {
    let mut output = vec![0; 32];
    (*ARGON_CONFIG).hash(&mut output, data, salt, &[], &[]);
    output
}

#[inline]
pub fn hash_sha256(data: &[u8], salt: &[u8]) -> Vec<u8> {
    let mut output = vec![0; 32];
    derive(PBKDF2_HMAC_SHA512, *SHA_REPS, salt, data, &mut output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_scrypt() {
        let hashed = hash_scrypt(&vec![1; 32], &vec![2; 32]);
        let expected: Vec<u8> = vec![28, 234, 128, 254, 84, 184, 1, 65, 151, 233, 171, 200, 65, 29, 106, 154, 65, 227, 136, 17, 6, 197, 135, 104, 124, 206, 31, 105, 12, 197, 92, 30];
        assert_eq!(expected, hashed);
    }

    #[test]
    fn test_hash_argon2i() {
        let hashed = hash_argon2i(&vec![1; 32], &vec![2; 32]);
        let expected: Vec<u8> = vec![114, 139, 48, 2, 98, 196, 133, 19, 232, 144, 6, 149, 44, 68, 116, 152, 233, 120, 110, 205, 15, 29, 180, 181, 4, 86, 84, 153, 228, 231, 106, 225];
        assert_eq!(expected, hashed);
    }

    #[test]
    fn test_hash_sha256() {
        let hashed = hash_sha256(&vec![1; 32], &vec![2; 32]);
        let expected: Vec<u8> = vec![89, 92, 124, 234, 51, 101, 87, 71, 223, 104, 235, 37, 116, 52, 18, 253, 105, 30, 196, 19, 174, 103, 43, 152, 200, 52, 241, 160, 102, 155, 118, 89];
        assert_eq!(expected, hashed);
    }
}
