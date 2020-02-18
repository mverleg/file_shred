use ::lazy_static::lazy_static;
use ::semver::Version;

lazy_static! {
    static ref CURRENT_VERSION: Version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
}

pub fn get_current_version() -> Version {
    CURRENT_VERSION.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_version() {
        assert!(*CURRENT_VERSION >= Version::parse("1.0.0").unwrap());
    }

    #[test]
    fn numbers_only() {
        assert_eq!(0, CURRENT_VERSION.build.len());
        assert_eq!(0, CURRENT_VERSION.pre.len());
    }
}
