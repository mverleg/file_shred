use ::semver::Version;
use ::lazy_static::lazy_static;
use crate::util::FedResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompressionAlg {
    Brotli,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyHashAlg {
    Sha256,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymmetricEncryptionAlg {
    None, //TODO @mark: TEMPORARY! REMOVE THIS!
}

#[derive(Debug, PartialEq, Eq)]
pub struct Strategy {
    stretch_count: u64,
    compression_algorithm: CompressionAlg,
    key_hash_algorithms: Vec<KeyHashAlg>,
    symmetric_algorithms: Vec<SymmetricEncryptionAlg>,
}

lazy_static! {
    static ref STRATEGY_1_0_0: Strategy = Strategy {
        //TODO @mark: more algorithms
        stretch_count: 1,
        compression_algorithm: CompressionAlg::Brotli,
        key_hash_algorithms: vec![KeyHashAlg::Sha256],
        symmetric_algorithms: vec![SymmetricEncryptionAlg::None],
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
