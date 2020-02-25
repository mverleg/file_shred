use ::semver::Version;

use crate::files::Checksum;
use crate::header::get_version_strategy;
use crate::header::Strategy;
use crate::key::Salt;
use crate::util::FedResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    version: Version,
    salt: Salt,
    strategy: &'static Strategy,
    checksum: Checksum,
}

impl Header {
    pub fn new(version: Version, salt: Salt, checksum: Checksum, verbose: bool) -> FedResult<Self> {
        let strategy = get_version_strategy(&version, verbose)
            .map_err(|e| format!("version used to encrypt: {}", e))?;
        Ok(Header {
            version,
            salt,
            strategy,
            checksum,
        })
    }

    pub fn version(&self) -> &Version {
        &self.version
    }
    pub fn salt(&self) -> &Salt {
        &self.salt
    }
    pub fn strategy(&self) -> &'static Strategy {
        self.strategy
    }
    pub fn checksum(&self) -> &Checksum {
        &self.checksum
    }
}

pub const HEADER_MARKER: &str = "github.com/mverleg/file_endec";
pub const HEADER_VERSION_MARKER: &str = "v ";
pub const HEADER_SALT_MARKER: &str = "salt ";
pub const HEADER_CHECKSUM_MARKER: &str = "check ";
pub const HEADER_DATA_MARKER: &str = "data:";
