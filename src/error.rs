use std::{error::Error, fmt::Display};

/// Error type for the rcgal crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RcgalError {
    /// One of the input arguments is not finite.
    NotFiniteInput,
    /// The result cannot be represented in the output type.
    Overflow,
}

impl Display for RcgalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RcgalError {}
