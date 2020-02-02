use ::std::fmt::Debug;
use ::std::fmt::Error;
use ::std::fmt::Formatter;

use ::rand::Rng;

const SALT_LEN: usize = 128;  // multiple of 32

#[derive(Clone, Copy)]
pub struct Salt {
    pub salt: [u8; SALT_LEN],
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
    pub fn generate_random() -> Self {
        let mut long = [0u8; SALT_LEN];
        for i in 0..SALT_LEN {
            long[i] = rand::thread_rng().gen::<u8>();
        }
        Salt { salt: long }
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
        assert_eq!(&debug, "salt[199...15]");
    }

    #[test]
    fn generate_salt() {
        Salt::generate_random();
    }
}
