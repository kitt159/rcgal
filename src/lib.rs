//! # RCGAL

mod error;
mod point;
mod vector;

pub use error::RcgalError;
pub use point::Point;
pub use vector::Vector;

#[doc(hidden)]
#[cfg(test)]
#[macro_export]
macro_rules! rel_eq {
    ($left:expr, $right:expr) => {
        ::approx::assert_relative_eq!(
            $left,
            $right,
            epsilon = f64::MIN_POSITIVE,
            max_relative = f64::EPSILON
        );
    };
}
