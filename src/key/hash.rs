
//TODO @mark: change to &mut [u8]? does that work with all algorithms?

use crate::header::KeyHashAlg;

#[inline]
pub fn hash(data: &mut Vec<u8>, algorithm: &KeyHashAlg) {
    match algorithm {
        KeyHashAlg::SCrypt => hash_scrypt(data),
        KeyHashAlg::Argon2id => hash_argon2id(data),
        KeyHashAlg::Sha256 => ash_sha256(data),
    }
}

#[inline]
pub fn hash_scrypt(data: &mut [u8]) {
    unimplemented!()
}

#[inline]
pub fn hash_argon2id(data: &mut [u8]) {
    unimplemented!()
}

#[inline]
pub fn ash_sha256(data: &mut [u8]) {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use ::criterion::black_box;
    use ::criterion::Criterion;
    use ::criterion::criterion_group;

    use super::*;

    fn get_data() -> Vec<u8> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    }

    pub fn scrypt_benchmark(c: &mut Criterion) {
        let mut data = get_data();
        c.bench_function("scrypt", |b| b.iter(
            || hash_scrypt(black_box(&mut data))));
    }

    pub fn argon2id_benchmark(c: &mut Criterion) {
        let mut data = get_data();
        c.bench_function("argon2id", |b| b.iter(
            || hash_argon2id(black_box(&mut data))));
    }

    pub fn sha256_benchmark(c: &mut Criterion) {
        let mut data = get_data();
        c.bench_function("sha256", |b| b.iter(
            || ash_sha256(black_box(&mut data))));
    }

    criterion_group!(hash_bench,
            scrypt_benchmark,
            argon2id_benchmark,
            sha256_benchmark,
    );
}
