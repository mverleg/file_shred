use crate::header::KeyHashAlg;
use crate::key::hash::hash;
use crate::key::Key;
use crate::key::key::StretchKey;
use crate::key::Salt;

pub fn stretch_key(raw_key: &Key, salt: &Salt, stretch_count: u64, key_hash_algorithms: &Vec::<KeyHashAlg>) -> StretchKey {
    assert!(key_hash_algorithms.len() >= 1);
    let salt_bytes = salt.salt.to_le_bytes();
    let mut data = raw_key.key_data.clone().unsecure().as_bytes().to_owned();
    for key_hash_alg in key_hash_algorithms {
        println!("alg {:?} init", key_hash_alg);  //TODO @mark: TEMPORARY! REMOVE THIS!
        data.extend(&salt_bytes);
        data = hash(&mut data, &salt_bytes, key_hash_alg);
        for i in 0 .. stretch_count {
            println!("alg {:?} i = {}", key_hash_alg, i);  //TODO @mark: TEMPORARY! REMOVE THIS!
            data.extend(&salt_bytes);
            data.extend(&i.to_le_bytes());
            data = hash(&mut data, &salt_bytes, key_hash_alg);
        }
    }
    StretchKey::new(&data)
}
