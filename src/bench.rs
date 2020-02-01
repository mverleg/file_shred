use ::criterion::criterion_main;
use ::criterion::criterion_group;

#[cfg(test)]
mod hash {
    use ::criterion::black_box;
    use ::criterion::Criterion;

    use ::file_endec::key::hash::hash_argon2i;
    use ::file_endec::key::hash::hash_scrypt;
    use ::file_endec::key::hash::hash_sha256;

    fn get_data() -> Vec<u8> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    }

    fn get_salt() -> Vec<u8> {
        vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]
    }

    pub fn scrypt_benchmark(c: &mut Criterion) {
        let mut data = get_data();
        c.bench_function("scrypt", |b| b.iter(
            || hash_scrypt(black_box(&mut data), &get_salt())));
    }

    pub fn argon2id_benchmark(c: &mut Criterion) {
        let mut data = get_data();
        c.bench_function("argon2id", |b| b.iter(
            || hash_argon2i(black_box(&mut data), &get_salt())));
    }

    pub fn sha256_benchmark(c: &mut Criterion) {
        let mut data = get_data();
        c.bench_function("sha256", |b| b.iter(
            || hash_sha256(black_box(&mut data), &get_salt())));
    }
}

criterion_group!(hash_bench,
    hash::scrypt_benchmark,
//    hash::argon2id_benchmark,
    hash::sha256_benchmark,
);

criterion_main!(
    hash_bench,
);
