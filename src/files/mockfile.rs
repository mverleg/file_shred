pub fn generate_test_file_content_for_test(len: usize) -> Vec<u8> {
    let mut data = vec![0u8; len];
    let mut a: u32 = 1;
    let mut b: u32 = 1;
    #[allow(clippy::needless_range_loop)]
    for i in 0..len {
        let c = (a + b) % 256;
        data[i] = c as u8;
        a = b;
        b = c;
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        let data = generate_test_file_content_for_test(15_001);
        assert_eq!(15_001, data.len());
        assert!(data.contains(&0));
        assert!(data.contains(&127));
        assert!(data.contains(&255));
    }
}
