use ::std::fmt::Debug;
use ::std::fmt::Error;
use ::std::fmt::Formatter;

use ::rand::RngCore;
use ::rand::rngs::OsRng;

use crate::util::base64::base64str_to_u8s;
use crate::util::base64::u8s_to_base64str;
use crate::util::FedResult;

const SALT_LEN: usize = 64; // multiple of 32

#[derive(Clone)]
pub struct Salt {
    pub salt: [u8; SALT_LEN],
}

impl PartialEq for Salt {
    fn eq(&self, other: &Self) -> bool {
        if self.salt.len() != other.salt.len() {
            // Perhaps this is redundant, since they're currently statically sized...
            return false;
        }
        for i in 0..self.salt.len() {
            if self.salt[i] != other.salt[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Salt {}

impl Debug for Salt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("salt[")?;
        f.write_str(&format!("{}", self.salt[0]))?;
        f.write_str("...")?;
        f.write_str(&format!("{}", self.salt[self.salt.len() - 1]))?;
        f.write_str("]")
    }
}

impl Salt {
    pub fn generate_random() -> FedResult<Self> {
        let mut long = [0u8; SALT_LEN];
        OsRng.fill_bytes(&mut long);
        Ok(Salt { salt: long })
    }

    pub fn fixed_for_test(salt: u64) -> Self {
        // Iterator didn't work: salt.to_le_bytes().into_iter().cycle().take(SALT_LEN).collect::<Vec<u8>>()
        let mut repeated = [0u8; SALT_LEN];
        let input = salt.to_le_bytes();
        for i in 0..repeated.len() {
            repeated[i] = input[i % input.len()];
        }
        Salt { salt: repeated }
    }

    pub fn parse_base64(base64: &str, verbose: bool) -> FedResult<Self> {
        match base64str_to_u8s(base64) {
            Ok(salt_vec) => {
                if salt_vec.len() == 64 {
                    let mut salt = [0; SALT_LEN];
                    salt.clone_from_slice(&salt_vec);
                    Ok(Salt { salt })
                } else {
                    Err(if verbose {
                        format!("could not determine the salt used by fileenc that encrypted this file; got {} which is invalid because it has the wrong length", base64)
                    } else {
                        "could not determine the salt used by fileenc to encrypt this file".to_owned()
                    })
                }
            },
            Err(err) => Err(if verbose {
                format!("could not determine the salt used by fileenc that encrypted this file; got {} which is invalid, reason: {}", base64, err)
            } else {
                "could not determine the salt used by fileenc to encrypt this file".to_owned()
            }),
        }
    }

    pub fn as_base64(&self) -> String {
        u8s_to_base64str(&self.salt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_impl() {
        let salt = Salt::fixed_for_test(1_111_111_111_111_111_111);
        let debug = format!("{:?}", salt);
        assert_eq!("salt[199...15]", &debug);
    }

    #[test]
    fn generate_salt_entropy() {
        // Fails if all bytes are the same (or if generation fails).
        // This test could theoretically fail in extremely rare cases.
        let salt = Salt::generate_random().unwrap();
        let r = salt.salt[0];
        for i in 0..salt.salt.len() {
            if salt.salt[i] != r {
                return;
            }
        }
        panic!(); // Should have returned before here.
    }

    #[test]
    fn generate_salt_different() {
        // Fails if two subsequent salts are identical (or if generation fails).
        // This test could theoretically fail in extremely rare cases.
        let salt1 = Salt::generate_random().unwrap();
        let salt2 = Salt::generate_random().unwrap();
        assert!(salt1 != salt2);
    }
}
