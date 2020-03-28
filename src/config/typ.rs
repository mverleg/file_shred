#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,
    Normal,
    Debug,
}

impl Default for Verbosity {
    fn default() -> Self {
        Verbosity::Normal
    }
}

impl Verbosity {
    pub fn debug(self) -> bool {
        Verbosity::Debug == self
    }

    pub fn quiet(self) -> bool {
        Verbosity::Quiet == self
    }
}
