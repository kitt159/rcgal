#![warn(missing_docs)]

//! # RCGAL

mod point;

pub use point::Point;

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
