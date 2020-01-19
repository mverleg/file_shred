use ::lazy_static::lazy_static;
use ::semver::Version;

lazy_static! {
    pub static ref CURRENT_VERSION: Version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert!(CURRENT_VERSION.clone() >= Version::parse("1.0.0").unwrap());
    }
}
