use ::lazy_static::lazy_static;
use ::semver::Version;

lazy_static! {
    pub static ref CURRENT_VERSION: Version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
}

#[cfg(test)]
mod version {
    use super::*;

    #[test]
    fn test_minimum_version() {
        assert!(&*CURRENT_VERSION >= &Version::parse("1.0.0").unwrap());
    }

    #[test]
    fn test_numbers_only() {
        assert_eq!(0, CURRENT_VERSION.build.len());
        assert_eq!(0, CURRENT_VERSION.pre.len());
    }
}
