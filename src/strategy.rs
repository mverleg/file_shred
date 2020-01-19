use ::semver::Version;
use ::lazy_static::lazy_static;

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

pub fn get_version_strategy(version: Version) -> &'static Strategy {
    assert!(version >= Version::parse("1.0.0").unwrap());
    &*STRATEGY_1_0_0
}
