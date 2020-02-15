use ::criterion::criterion_group;
use ::criterion::criterion_main;

#[cfg(test)]
mod hash {

    use ::criterion::black_box;
    use ::criterion::Benchmark;
    use ::criterion::Criterion;

    use ::file_endec::header::strategy::get_current_version_strategy;
    use ::file_endec::key::hash::hash_argon2i;
    use ::file_endec::key::hash::hash_bcrypt;
    use ::file_endec::key::hash::hash_sha256;
    use ::file_endec::key::stretch::stretch_key;
    use ::file_endec::key::{Key, Salt};

    fn get_data() -> Vec<u8> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    }

    fn get_salt() -> Vec<u8> {
        vec![
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            0, 1, 0,
        ]
    }

    pub fn scrypt_benchmark(c: &mut Criterion) {
        c.bench(
            "bcrypt",
            Benchmark::new("bcrypt", |b| {
                b.iter(|| hash_bcrypt(black_box(&mut get_data()), &get_salt()))
            })
            .sample_size(20),
        );
    }

    pub fn argon2id_benchmark(c: &mut Criterion) {
        c.bench(
            "argon2id",
            Benchmark::new("argon2id", |b| {
                b.iter(|| hash_argon2i(black_box(&mut get_data()), &get_salt()))
            })
            .sample_size(20),
        );
    }

    pub fn sha256_benchmark(c: &mut Criterion) {
        c.bench(
            "sha256_hash",
            Benchmark::new("sha256_hash", |b| {
                b.iter(|| hash_sha256(black_box(&mut get_data()), &get_salt()))
            })
            .sample_size(20),
        );
    }

    pub fn stretch_benchmark(c: &mut Criterion) {
        c.bench(
            "stretch",
            Benchmark::new("stretch", |b| {
                b.iter(|| {
                    let strat = get_current_version_strategy(true);
                    stretch_key(
                        &Key::new(&"MY secret p@ssw0rd"),
                        &Salt::static_for_test(123_456_789),
                        strat.stretch_count,
                        &strat.key_hash_algorithms,
                    )
                })
            })
            .sample_size(5),
        );
    }
}

#[cfg(test)]
mod encrypt {

    use ::criterion::black_box;
    use ::criterion::Benchmark;
    use ::criterion::Criterion;

    use ::file_endec::files::mockfile::generate_test_file_content_for_test;

    use ::file_endec::key::key::StretchKey;

    use ::file_endec::key::Salt;

    use ::file_endec::symmetric::encrypt::encrypt_aes256;

    pub fn encrypt_aes256_benchmark(c: &mut Criterion) {
        c.bench(
            "enc_aes256",
            Benchmark::new("enc_aes256", |b| {
                b.iter(|| {
                    let key = StretchKey::mock_stretch("1_s3cr3t_p@55w0rd!!".as_bytes());
                    let salt = Salt::static_for_test(123_456_789_123_456_789);
                    let input = generate_test_file_content_for_test(1_000_000);
                    let actual = encrypt_aes256(black_box(input), &key, &salt).unwrap();
                    let expected_start = &[81, 163, 93, 212, 203, 139, 62, 17];
                    assert_eq!(&actual[..8], expected_start);
                })
            })
            .sample_size(10),
        );
    }
}

//TODO @mark: fully encrypt and decrypt large file

criterion_group!(
    hash_bench,
    hash::scrypt_benchmark,
    hash::argon2id_benchmark,
    hash::sha256_benchmark,
    hash::stretch_benchmark,
);

criterion_group!(encrypt_bench, encrypt::encrypt_aes256_benchmark,);

criterion_main!(
    //    hash_bench,
    encrypt_bench,
);
