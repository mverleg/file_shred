use ::lazy_static::lazy_static;
use ::semver::Version;
use crate::header::Strategy;
use crate::header::CompressionAlg;
use crate::header::KeyHashAlg;
use crate::header::SymmetricEncryptionAlg;

lazy_static! {
    static ref CURRENT_VERSION: Version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert!(CURRENT_VERSION.clone() >= Version::parse("1.0.0").unwrap());
    }
}

lazy_static! {
    static ref STRATEGY_1_0_0: Strategy = Strategy {
        //TODO @mark: more algorithms
        stretch_count: 1,
        compression_algorithm: CompressionAlg::Brotli,
        key_hash_algorithms: vec![KeyHashAlg::sha256],
        symmetric_algorithms: vec![SymmetricEncryptionAlg::None],
    };
}


pub fn get_version_strategy(version: Version) -> &'static Strategy {
    assert!(version >= Version::parse("1.0.0").unwrap());
    &*STRATEGY_1_0_0
}
