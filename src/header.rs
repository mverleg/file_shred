
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompressionAlg {
    Brotli,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyHashAlg {

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymmetricEncryptionAlg {

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    version: (u32, u32, u32),
    salt: u64,
    stretch_count: u64,
    compression_algorithm: CompressionAlg,
    key_hash_algorithms: ArrayVec<[KeyHashAlg; 4]>,
    symmetric_algorithms: ArrayVec<[SymmetricEncryptionAlg; 4]>,
}
