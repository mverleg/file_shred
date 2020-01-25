
#[derive(Debug, Clone, Copy)]
pub struct Salt {
    pub salt: u64,
}

impl Salt {
    pub fn generate_random() -> Self {
        //TODO @mark:
        unimplemented!()
    }

    pub fn static_for_test(salt: u64) -> Self {
        Salt { salt }
    }
}
