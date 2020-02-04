use ::std::fmt::Display;
use ::std::fmt::Error;
use ::std::fmt::Formatter;
use ::std::num::NonZeroU32;
use ::std::hash::Hasher;

use ::twox_hash::XxHash64;
use ring::pbkdf2::{derive, PBKDF2_HMAC_SHA512};

use crate::util::FedResult;
use crate::util::util::{base64str_to_u8s, u8s_to_base64str};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChecksumType {
    // Hash the file content with xxhash, then sha256 the result for irreversibility, and express as base64.
    #[allow(non_camel_case_types)]
    Xxhash_Sha256_b64,
}

impl ChecksumType {
    pub fn parse(input: &str) -> FedResult<Self> {
        if "xx_sha256" == input {
            Ok(ChecksumType::Xxhash_Sha256_b64)
        } else {
            Err(format!("unrecognized checksum type: '{}'", input))
        }
    }
}

impl Display for ChecksumType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            ChecksumType::Xxhash_Sha256_b64 => f.write_str("xx_sha256")?,
        };
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checksum {
    typ: ChecksumType,
    value: Vec<u8>,
}

impl Checksum {
    pub fn fixed_for_test(data: Vec<u8>) -> Self {
        Checksum {
            typ: ChecksumType::Xxhash_Sha256_b64,
            value: data,
        }
    }

    pub fn parse(input: &str) -> FedResult<Self> {
        if &input[..9] == "xx_sha256" {
            return Ok(Checksum {
                typ: ChecksumType::Xxhash_Sha256_b64,
                value: base64str_to_u8s(&input[10..])?,
            });
        }
        Err(format!("failed to parse checksum format: {}", input.split(" ").next().unwrap()))
    }
}

impl Display for Checksum {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(&self.typ.to_string())?;
        f.write_str(" ")?;
        f.write_str(&u8s_to_base64str(&self.value))?;
        Ok(())
    }
}

pub fn calculate_checksum(data: &[u8]) -> Checksum {
    let mut hasher = XxHash64::with_seed(5_771_919_056_451_745_621);
    for b in data {
        hasher.write_u8(*b);
    }
    let xxhash = hasher.finish().to_le_bytes();
    let mut shahash = vec![0; 16];
    derive(PBKDF2_HMAC_SHA512, NonZeroU32::new(1).unwrap(), &[], &xxhash, &mut shahash);
    Checksum {
        typ: ChecksumType::Xxhash_Sha256_b64,
        value: shahash,
    }
}

#[cfg(test)]
mod tests {
    use crate::files::mockfile::generate_test_file_content_for_test;

    use super::*;

    #[test]
    fn parse() {
        let input = "xx_sha256 AQIDBAAABQYHCP-qWg";
        let parsed = Checksum::parse(input).unwrap();
        let expected = Checksum::fixed_for_test(vec![1, 2, 3, 4, 0, 0, 5, 6, 7, 8, 255, 170, 90]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn calculate() {
        let data = generate_test_file_content_for_test(15_001);
        let checksum = calculate_checksum(&data);
        assert_eq!(checksum.value, vec![219, 36, 108, 103, 132, 201, 242, 88, 202, 217, 207, 138, 186, 93, 68, 203]);
    }
}
