use crate::header::KeyHashAlg;
use crate::key::hash::hash;
use crate::key::Key;
use crate::key::key::StretchKey;
use crate::key::Salt;

pub fn stretch_key(
    raw_key: &Key,
    salt: &Salt,
    stretch_count: u64,
    key_hash_algorithms: &[KeyHashAlg],
) -> StretchKey {
    assert!(!key_hash_algorithms.is_empty());
    let salt_bytes = salt.salt;
    let mut data = raw_key.key_data.clone().unsecure().as_bytes().to_owned();
    for key_hash_alg in key_hash_algorithms {
        data = hash(&data, &salt_bytes, key_hash_alg);
        for i in 0..stretch_count {
            data.extend(&i.to_le_bytes());
            data = hash(&data, &salt_bytes, key_hash_alg);
        }
    }
    StretchKey::new(&data)
}

#[cfg(test)]
mod tests {
    #[cfg(not(debug_assertions))]
    #[test]
    fn stratch_test_password() {
        let strat = get_current_version_strategy(true);
        stretch_key(
            &Key::new(&"MY secret p@ssw0rd"),
            &Salt::static_for_test(123_456_789),
            strat.stretch_count,
            &strat.key_hash_algorithms,
        );
    }

    #[cfg(debug_assertions)]
    #[test]
    #[ignore]
    fn stratch_test_password() {
        panic!("Test skipped in debug mode, because it is really slow");
    }
}
