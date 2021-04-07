#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Metasymbol {
    Epsilon,
    Failure,
    Any,
    All,
    Omit,
}

impl Metasymbol {
    pub fn epsilon() -> Self {
        Self::Epsilon
    }

    pub fn failed() -> Self {
        Self::Failure
    }

    pub fn any() -> Self {
        Self::Any
    }

    pub fn all() -> Self {
        Self::All
    }

    pub fn omit() -> Self {
        Self::Omit
    }
}
