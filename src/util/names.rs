
const CHARS: &'static [u8] = b"0123456789abcdefghijklmnopqrstuvwxyz_-";

pub fn generate_name(number: u32) -> String {
    #[allow(non_snake_case)]
    let LEN = CHARS.len() as u32;
    #[allow(non_snake_case)]  // assume the compiler recognizes this as a constant
    let ITER_COUNT: u32 = (std::u32::MAX as f64).log(LEN as f64).floor() as u32;

    dbg!(&ITER_COUNT);  //TODO @mverleg: should be 6
    let mut remainder = number + 1;
    let mut power = ITER_COUNT;
    let mut name = String::new();
    while remainder > 0 {
        dbg!(remainder);  //TODO @mverleg: remove
        dbg!(power);  //TODO @mverleg: remove
        let scale = CHARS.len().pow(power) as u32;
        if remainder < scale {
            power -= 1;
            continue;
        }
        let index = (remainder / scale) as usize - 1;
        dbg!(index);  //TODO @mverleg: remove
        name.push(CHARS[index] as char);
        dbg!(&name);  //TODO @mverleg: remove
        remainder -= scale;
    }
    name
}

//TODO @mark: test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_name() {
        let name = generate_name(0);
        assert_eq!(name, "0");
    }

    #[test]
    fn two_letter_name() {
        let name = generate_name(38);
        assert_eq!(name, "00");
    }

    #[test]
    fn four_letter_name() {
        let name = generate_name(2_085_136);
        assert_eq!(name, "0000");
    }
}
