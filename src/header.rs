use ::semver::Version;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strategy {
    stretch_count: u64,
    compression_algorithm: CompressionAlg,
    key_hash_algorithms: Vec<KeyHashAlg>,
    symmetric_algorithms: Vec<SymmetricEncryptionAlg>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    version: Version,
    salt: u64,
    strategy: Strategy,
    checksum: u64,
}
