use std::cmp::max;

const CHARS: &'static [u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

pub fn generate_name(number: u64) -> String {
    #[allow(non_snake_case)]
    let COUNT = CHARS.len() as usize;
    // #[allow(non_snake_case)]  // assuming the compiler recognizes this as a constant
    // let ITER_COUNT: u32 = (std::u32::MAX as f64).log(LEN as f64).floor() as u32;

    let name_len = (number as f64 + 1.0).log(COUNT as f64).ceil() as u32;
    // dbg!(name_len);  //TODO @mverleg: remove
    // dbg!(number);  //TODO @mverleg: remove
    let mut name = String::new();
    for power in (0..name_len).rev() {
        // dbg!(power);  //TODO @mverleg: remove
        let scale = COUNT.pow(power) as u64;
        // dbg!(scale);  //TODO @mverleg: remove
        let index = ((number / scale) % COUNT as u64) as usize;
        // dbg!(index);  //TODO @mverleg: remove
        name.push(CHARS[index] as char);
    }

    // dbg!(&ITER_COUNT);  //TODO @mverleg: should be 6
    // let mut remainder = number + 1;
    // let mut power = ITER_COUNT;
    // let mut name = String::new();
    // while remainder > 0 {
    //     dbg!(remainder);  //TODO @mverleg: remove
    //     dbg!(power);  //TODO @mverleg: remove
    //     let scale = CHARS.len().pow(power) as u32;
    //     if remainder < scale {
    //         power -= 1;
    //         continue;
    //     }
    //     let index = (remainder / scale) as usize - 1;
    //     dbg!(index);  //TODO @mverleg: remove
    //     name.push(CHARS[index] as char);
    //     dbg!(&name);  //TODO @mverleg: remove
    //     remainder -= scale;
    // }
    name
}

//TODO @mark: test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]  //TODO @mark: TEMPORARY! REMOVE THIS!
    fn tmp() {
        for i in 0..40 {
            println!("{}", generate_name(i));
        }
        unimplemented!()
    }

    #[test]
    fn first_name() {
        let name = generate_name(0);
        assert_eq!(name, "0");
    }

    #[test]
    fn last_single_letter_name() {
        let name = generate_name(35);
        assert_eq!(name, "z");
    }

    #[test]
    fn two_letter_name() {
        let name = generate_name(36);
        assert_eq!(name, "00");
    }

    #[test]
    fn three_letter_name() {
        let name = generate_name(36 * 36);
        assert_eq!(name, "000");
    }

    #[test]
    fn four_letter_name() {
        let name = generate_name(36 * 36 * 36);
        assert_eq!(name, "0000");
    }
}
