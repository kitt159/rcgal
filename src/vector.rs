type NaVec2 = nalgebra::Vector2<f64>;

/// Structure representing a 2D vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector {
    inner: NaVec2,
}

impl Vector {
    /// Creates a new 2D vector from x and y coordinates.
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            inner: NaVec2::new(x, y),
        }
    }

    /// Returns the x coordinate.
    pub fn x(&self) -> f64 {
        self.inner.x
    }

    /// Returns the y coordinate.
    pub fn y(&self) -> f64 {
        self.inner.y
    }

    /// Returns the length of the vector.
    pub fn norm(&self) -> f64 {
        self.inner.x.hypot(self.inner.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rel_eq;
    use std::f64::consts as C;

    #[test]
    fn vector_2d_has_xy() {
        let v = Vector::new(1.1, 2.2);
        rel_eq!(v.x(), 1.1);
        rel_eq!(v.y(), 2.2);
        let v = Vector::new(3.3, 4.4);
        rel_eq!(v.x(), 3.3);
        rel_eq!(v.y(), 4.4);
    }

    #[test]
    fn vector_2d_norm_simple() {
        let v = Vector::new(0.0, 0.0);
        rel_eq!(v.norm(), 0.0);
        let v = Vector::new(3.0, 4.0);
        rel_eq!(v.norm(), 5.0);
        let v = Vector::new(1.0, 1.0);
        rel_eq!(v.norm(), C::SQRT_2);
    }

    #[test]
    fn vector_2d_norm_big() {
        let v = Vector::new(1e300, 0.0);
        rel_eq!(v.norm(), 1e300);
        let v = Vector::new(0.0, -2e300);
        rel_eq!(v.norm(), 2e300);
    }

    #[test]
    fn vector_2d_norm_small() {
        let v = Vector::new(1e-300, 0.0);
        rel_eq!(v.norm(), 1e-300);
        let v = Vector::new(0.0, -2e-300);
        rel_eq!(v.norm(), 2e-300);
    }

    #[test]
    fn vector_2d_norm_overflow() {
        let v = Vector::new(f64::MAX, f64::MAX);
        assert!(v.norm().is_infinite());
    }

    #[test]
    fn vector_2d_norm_invalid_arguments() {
        let v = Vector::new(0.0, f64::INFINITY);
        assert!(v.norm().is_infinite());
        let v = Vector::new(0.0, f64::NEG_INFINITY);
        assert!(v.norm().is_infinite());
        let v = Vector::new(0.0, f64::NAN);
        assert!(v.norm().is_nan());
        let v = Vector::new(f64::INFINITY, 0.0);
        assert!(v.norm().is_infinite());
        let v = Vector::new(f64::INFINITY, f64::INFINITY);
        assert!(v.norm().is_infinite());
        let v = Vector::new(f64::INFINITY, f64::NEG_INFINITY);
        assert!(v.norm().is_infinite());
        let v = Vector::new(f64::INFINITY, f64::NAN);
        assert!(v.norm().is_infinite());
        let v = Vector::new(f64::NEG_INFINITY, 0.0);
        assert!(v.norm().is_infinite());
        let v = Vector::new(f64::NEG_INFINITY, f64::INFINITY);
        assert!(v.norm().is_infinite());
        let v = Vector::new(f64::NEG_INFINITY, f64::NEG_INFINITY);
        assert!(v.norm().is_infinite());
        let v = Vector::new(f64::NEG_INFINITY, f64::NAN);
        assert!(v.norm().is_infinite());
        let v = Vector::new(f64::NAN, 0.0);
        assert!(v.norm().is_nan());
        let v = Vector::new(f64::NAN, f64::INFINITY);
        assert!(v.norm().is_infinite());
        let v = Vector::new(f64::NAN, f64::NEG_INFINITY);
        assert!(v.norm().is_infinite());
        let v = Vector::new(f64::NAN, f64::NAN);
        assert!(v.norm().is_nan());
    }
}
