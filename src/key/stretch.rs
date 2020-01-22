use crate::key::Key;
use crate::key::Salt;
use crate::header::KeyHashAlg;
use crate::key::hash::hash;

pub fn stretch_key(raw_key: &Key, seed: &Salt, stretch_count: u64, key_hash_algorithms: &Vec::<KeyHashAlg>) {

    let mut key = raw_key.key_data.clone().unsecure().as_bytes().to_owned();
    for key_hash_alg in key_hash_algorithms {
        hash(&mut key, key_hash_alg);
        for _ in 0 .. stretch_count {
            hash(&mut key, key_hash_alg);
        }
    }

}
