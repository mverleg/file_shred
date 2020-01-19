use ::semver::Version;

use crate::header::Strategy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    version: Version,
    salt: u64,
    strategy: &'static Strategy,
    checksum: u64,
}

impl Header {
    pub fn new(
        version: Version,
        salt: u64,
        strategy: &'static Strategy,
        checksum: u64,
    ) -> Self {
        Header {
            version,
            salt,
            strategy,
            checksum,
        }
    }
}

pub const HEADER_MARKER: &str = "github.com/mverleg/file_endec";
