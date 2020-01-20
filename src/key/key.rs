use ::secstr::SecUtf8;
use ::zxcvbn::Entropy;
use ::zxcvbn::zxcvbn;

#[derive(Debug)]
pub struct Key {
    pub key_data: SecUtf8,
    pub strength: Entropy,
}

impl Key {
    pub fn new(key_data: &str) -> Self {
        let strength = zxcvbn(key_data, &[]).unwrap();
        Key {
            key_data: SecUtf8::from(key_data),
            strength,
        }
    }

    pub fn is_strong(&self) -> bool {
        self.strength.score() >= 3
    }

    pub fn time_to_crack(&self) -> String {
        format!("{}", self.strength.crack_times().offline_fast_hashing_1e10_per_second())
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.key_data == other.key_data
    }
}

impl Eq for Key {}
