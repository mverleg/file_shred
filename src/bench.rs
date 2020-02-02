use ::criterion::criterion_main;
use ::criterion::criterion_group;

#[cfg(test)]
mod hash {
    use ::criterion::black_box;
    use ::criterion::Criterion;

    use ::file_endec::key::hash::hash_argon2i;
    use ::file_endec::key::hash::hash_scrypt;
    use ::file_endec::key::hash::hash_sha256;
    use criterion::Benchmark;
    use std::time::Duration;
    use file_endec::key::stretch::stretch_key;
    use file_endec::key::{Key, Salt};
    use file_endec::header::strategy::get_current_version_strategy;

    fn get_data() -> Vec<u8> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    }

    fn get_salt() -> Vec<u8> {
        vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]
    }

    pub fn scrypt_benchmark(c: &mut Criterion) {
        c.bench("scrypt",
            Benchmark::new("scrypt", |b| b.iter(||
                hash_scrypt(black_box(&mut get_data()), &get_salt()))
            ).sample_size(20),
        );
    }

    pub fn argon2id_benchmark(c: &mut Criterion) {
        c.bench("argon2id",
            Benchmark::new("argon2id", |b| b.iter(||
                hash_argon2i(black_box(&mut get_data()), &get_salt()))
            ).sample_size(20),
        );
    }

    pub fn sha256_benchmark(c: &mut Criterion) {
        c.bench("sha256",
            Benchmark::new("sha256", |b| b.iter(||
                hash_sha256(black_box(&mut get_data()), &get_salt()))
            ).sample_size(20),
        );
    }

    pub fn stretch_benchmark(c: &mut Criterion) {
        c.bench("stretch",
                Benchmark::new("stretch", |b| b.iter(|| {
                    let strat = get_current_version_strategy(true);
                    stretch_key(
                        &Key::new(&"MY secret p@ssw0rd"),
                        &Salt::static_for_test(123_456_789),
                        strat.stretch_count,
                        &strat.key_hash_algorithms,
                    )}),
                ).sample_size(5),
        );
    }
}

criterion_group!(hash_bench,
//    hash::scrypt_benchmark,
//    hash::argon2id_benchmark,
//    hash::sha256_benchmark,
    hash::stretch_benchmark,
);

criterion_main!(
    hash_bench,
);
