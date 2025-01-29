#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,
    #[default]
    Normal,
    Debug,
}

impl Verbosity {
    pub fn debug(self) -> bool {
        Verbosity::Debug == self
    }

    pub fn quiet(self) -> bool {
        Verbosity::Quiet == self
    }
}
