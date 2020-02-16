use ::std::fmt::Debug;
use ::std::fmt::Error;
use ::std::fmt::Formatter;

use ::rand::rngs::OsRng;
use ::rand::RngCore;

use crate::util::FedResult;

const SALT_LEN: usize = 128; // multiple of 32

#[derive(Clone, Copy)]
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

    pub fn static_for_test(salt: u64) -> Self {
        // Iterator didn't work: salt.to_le_bytes().into_iter().cycle().take(SALT_LEN).collect::<Vec<u8>>()
        let mut repeated = [0u8; SALT_LEN];
        let input = salt.to_le_bytes();
        for i in 0..repeated.len() {
            repeated[i] = input[i % input.len()];
        }
        Salt { salt: repeated }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_impl() {
        let salt = Salt::static_for_test(1_111_111_111_111_111_111);
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
