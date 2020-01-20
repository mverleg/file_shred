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
        let strength = zxcvbn(config.key, &[]).unwrap();
        Key {
            key_data: SecUtf8::from(key_data),
            strength,
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.key_data == other.key_data
    }
}

impl Eq for Key {}
