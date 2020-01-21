use crate::key::Key;
use crate::key::Salt;
use crate::header::KeyHashAlg;

pub fn stretch_key(raw_key: &Key, seed: &Salt, stretch_count: u64, key_hash_algorithms: &Vec::<KeyHashAlg>) {

    let mut key = raw_key.key_data.clone().unsecure().as_bytes().to_owned();
    for _ in stretch_count {
        key = hash(key)
    }

}
