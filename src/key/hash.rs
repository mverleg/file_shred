use ::std::num::NonZeroU32;

use ::argon2rs::Argon2;
use ::argon2rs::Variant;
use ::lazy_static::lazy_static;
use ::ring::pbkdf2::derive;
use ::ring::pbkdf2::PBKDF2_HMAC_SHA512;

use ::bcrypt;

use crate::header::KeyHashAlg;

#[inline]
pub fn hash(data: &[u8], salt: &[u8], algorithm: &KeyHashAlg) -> Vec<u8> {
    match algorithm {
        KeyHashAlg::BCrypt => hash_bcrypt(data, salt),
        KeyHashAlg::Argon2i => hash_argon2i(data, salt),
        KeyHashAlg::Sha512 => hash_sha256(data, salt),
    }
}

lazy_static! {
    // NOTE: if the hash parameters ever change, then KeyHashAlg must be extended to reflect that,
    // to make sure that anything encrypted before can still be decrypted.
    static ref BCRYPT_COST: u32 = 10;
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
pub fn hash_bcrypt(data: &[u8], salt: &[u8]) -> Vec<u8> {
    // Note that this returns a string, which is a combination of the base64 key, and metadata like salt.
    // Also note that 0-bytes are now allowed in the input.
    let mut nonzero = data.to_vec();
    //TODO: use SIMD to do this check faster?
    nonzero
        .iter_mut()
        .enumerate()
        .filter(|(_, v)| **v == 0u8)
        .for_each(|(i, v)| *v = 1 + (i % 255) as u8);
    bcrypt::hash_with_salt(&nonzero, *BCRYPT_COST, &salt[..16])
        .unwrap()
        .to_string()
        .into_bytes()
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

#[inline]
pub fn fastish_hash(data: &[u8]) -> Vec<u8> {
    let mut output = vec![0; 32];
    derive(
        PBKDF2_HMAC_SHA512,
        NonZeroU32::new(1).unwrap(),
        b"&8KQTJKpIMdz7Da*4weK$vuzVEd=mtIT",
        data,
        &mut output,
    );
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_bcrypt() {
        let hashed = hash_bcrypt(&[1; 32], &[2; 32]);
        let expected: Vec<u8> = vec![
            36, 50, 121, 36, 49, 48, 36, 46, 101, 71, 65, 46, 101, 71, 65, 46, 101, 71, 65, 46,
            101, 71, 65, 46, 101, 71, 65, 46, 101, 66, 114, 86, 57, 105, 106, 106, 103, 68, 70, 75,
            79, 97, 88, 118, 84, 121, 97, 97, 48, 115, 52, 67, 47, 54, 97, 109, 79, 48, 89, 117,
        ];
        assert_eq!(expected, hashed);
    }

    #[test]
    fn test_hash_bcrypt_0() {
        let hashed = hash_bcrypt(&[0; 32], &[0; 16]);
        let expected: Vec<u8> = vec![
            36, 50, 121, 36, 49, 48, 36, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46, 46,
            46, 46, 46, 46, 46, 46, 46, 46, 99, 118, 56, 107, 82, 56, 47, 81, 116, 102, 76, 79, 50,
            50, 55, 98, 106, 49, 104, 56, 66, 69, 74, 112, 47, 55, 67, 109, 111, 77, 121,
        ];
        assert_eq!(expected, hashed);
    }

    #[test]
    fn test_hash_argon2i() {
        let hashed = hash_argon2i(&[1; 32], &[2; 32]);
        let expected: Vec<u8> = vec![
            114, 139, 48, 2, 98, 196, 133, 19, 232, 144, 6, 149, 44, 68, 116, 152, 233, 120, 110,
            205, 15, 29, 180, 181, 4, 86, 84, 153, 228, 231, 106, 225,
        ];
        assert_eq!(expected, hashed);
    }

    #[test]
    fn test_hash_argon2i_0() {
        let hashed = hash_argon2i(&[0; 32], &[0; 16]);
        let expected: Vec<u8> = vec![
            48, 210, 130, 114, 168, 121, 20, 79, 3, 184, 46, 80, 43, 161, 165, 121, 68, 175, 154,
            87, 128, 226, 23, 244, 222, 136, 41, 30, 92, 110, 88, 223,
        ];
        assert_eq!(expected, hashed);
    }

    #[test]
    fn test_hash_sha256() {
        let hashed = hash_sha256(&[1; 32], &[2; 32]);
        let expected: Vec<u8> = vec![
            89, 92, 124, 234, 51, 101, 87, 71, 223, 104, 235, 37, 116, 52, 18, 253, 105, 30, 196,
            19, 174, 103, 43, 152, 200, 52, 241, 160, 102, 155, 118, 89,
        ];
        assert_eq!(expected, hashed);
    }

    #[test]
    fn test_hash_sha256_0() {
        let hashed = hash_sha256(&[0; 32], &[0; 16]);
        let expected: Vec<u8> = vec![
            127, 219, 200, 223, 214, 170, 211, 3, 75, 60, 208, 193, 156, 151, 72, 51, 164, 78, 227,
            245, 160, 150, 122, 232, 234, 10, 198, 186, 236, 182, 37, 204,
        ];
        assert_eq!(expected, hashed);
    }
}
