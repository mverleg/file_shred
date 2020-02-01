
//TODO @mark: change to &mut [u8]? does that work with all algorithms?

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
    // https://crypto.stackexchange.com/questions/35423
    static ref SCRYPT_PARAMS: ScryptParams = ScryptParams::new(14, 8, 1);
}

lazy_static! {
    static ref ARGON_CONFIG: Argon2 = Argon2::default(Variant::Argon2i);
}

#[inline]
pub fn hash_scrypt(data: &[u8], salt: &[u8]) -> Vec<u8> {
    let mut output = vec![0; 32];
    //TODO @mark: put all changeable params in header
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
    //TODO @mark: put repetitions in header
    derive(PBKDF2_HMAC_SHA512, NonZeroU32::new(3000).unwrap(), salt, data, &mut output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_scrypt() {
        for i in 0..100 {
            let hashed = hash_scrypt(&vec![1; 32], &vec![2; 32]);
            let expected: Vec<u8> = vec![28, 234, 128, 254, 84, 184, 1, 65, 151, 233, 171, 200, 65, 29, 106, 154, 65, 227, 136, 17, 6, 197, 135, 104, 124, 206, 31, 105, 12, 197, 92, 30];
            assert_eq!(expected, hashed);
            println!("{}", i);  //TODO @mark: TEMPORARY! REMOVE THIS!
        }
    }

    #[test]
    fn test_hash_argon2id() {
        for i in 0..100 {
            let hashed = hash_argon2i(&vec![1; 32], &vec![2; 32]);
            let expected: Vec<u8> = vec![154, 167, 243, 76, 224, 11, 190, 190, 16, 53, 5, 112, 123, 111, 242, 242, 217, 204, 29, 52, 124, 27, 170, 145, 157, 167, 52, 43, 82, 205, 141, 89];
            assert_eq!(expected, hashed);
            println!("{}", i);  //TODO @mark: TEMPORARY! REMOVE THIS!
        }
    }

    #[test]
    fn test_hash_sha256() {
        for i in 0..100 {
            let hashed = hash_sha256(&vec![1; 32], &vec![2; 32]);
            let expected: Vec<u8> = vec![188, 243, 63, 250, 102, 212, 13, 123, 200, 237, 71, 176, 152, 157, 122, 117, 92, 128, 226, 83, 139, 63, 234, 131, 207, 209, 204, 26, 227, 96, 247, 8];
            assert_eq!(expected, hashed);
            println!("{}", i);  //TODO @mark: TEMPORARY! REMOVE THIS!
        }
    }
}
