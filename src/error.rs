use std::{convert::Infallible, error::Error, fmt::Display};

/// Error type for the rcgal crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RcgalError {
    /// One of the input arguments is not finite.
    NotFiniteInput,
    /// One of the input arguments does not meet the requirements.
    InvalidInput,
    /// The result cannot be represented in the output type.
    Overflow,
}

impl Display for RcgalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RcgalError {}

impl From<Infallible> for RcgalError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
