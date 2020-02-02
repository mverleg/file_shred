use ::lazy_static::lazy_static;
use ::semver::Version;

use crate::util::FedResult;
use crate::util::version::CURRENT_VERSION;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompressionAlg {
    Brotli,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyHashAlg {
    SCrypt,
    Argon2i,
    Sha512,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymmetricEncryptionAlg {
    Aes512,
    Twofish,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Strategy {
    pub stretch_count: u64,
    pub compression_algorithm: CompressionAlg,
    pub key_hash_algorithms: Vec<KeyHashAlg>,
    pub symmetric_algorithms: Vec<SymmetricEncryptionAlg>,
}

lazy_static! {
    static ref STRATEGY_1_0_0: Strategy = Strategy {
        //TODO @mark: more algorithms
        stretch_count: 5,
        compression_algorithm: CompressionAlg::Brotli,
        key_hash_algorithms: vec![KeyHashAlg::SCrypt, KeyHashAlg::Argon2i, KeyHashAlg::Sha512],
        symmetric_algorithms: vec![SymmetricEncryptionAlg::Aes512, SymmetricEncryptionAlg::Twofish],
    };
}

/// Get the encryption strategy used for a specific code version.
pub fn get_version_strategy(version: &Version, verbose: bool) -> FedResult<&'static Strategy> {
    // This should return the strategy for all old versions - don't delete any, just add new ones!
    if version < &Version::parse("1.0.0").unwrap() {
        return Err(match verbose {
            true => "non-existent version".to_owned(),
            false => format!("non-existent version {} (minimum is 1.0.0)", version),
        })
    }
    Ok(&*STRATEGY_1_0_0)
}

pub fn get_current_version_strategy(verbose: bool) -> &'static Strategy {
    get_version_strategy(&*CURRENT_VERSION, verbose).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_version_strategy() {
        get_current_version_strategy(true);
        get_current_version_strategy(false);
    }
}
