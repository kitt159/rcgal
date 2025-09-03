use crate::RcgalError;

/// Structure representing a 2D vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector {
    inner: NaVec2,
}

impl Vector {
    /// Creates a new 2D vector from x and y coordinates.
    pub fn new(x: f64, y: f64) -> Result<Self, RcgalError> {
        NaVec2::new(x, y).try_into()
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
    pub fn norm(&self) -> Result<f64, RcgalError> {
        let norm = self.inner.x.hypot(self.inner.y);
        norm.is_finite().then_some(norm).ok_or(RcgalError::Overflow)
    }
}

pub(crate) type NaVec2 = nalgebra::Vector2<f64>;

impl TryFrom<NaVec2> for Vector {
    type Error = RcgalError;
    fn try_from(value: NaVec2) -> Result<Self, Self::Error> {
        (value.x.is_finite() && value.y.is_finite())
            .then_some(Self { inner: value })
            .ok_or(RcgalError::NotFiniteInput)
    }
}

impl From<Vector> for NaVec2 {
    fn from(value: Vector) -> Self {
        value.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rel_eq;
    use std::f64::consts as C;

    #[test]
    fn vector_2d_has_xy() {
        let v = Vector::new(1.1, 2.2).unwrap();
        rel_eq!(v.x(), 1.1);
        rel_eq!(v.y(), 2.2);
        let v = Vector::new(3.3, 4.4).unwrap();
        rel_eq!(v.x(), 3.3);
        rel_eq!(v.y(), 4.4);
    }

    #[test]
    fn vector_2d_invalid_arguments() {
        let e = RcgalError::NotFiniteInput;
        let v = Vector::new(f64::NAN, 0.0);
        assert_eq!(v.unwrap_err(), e);
        let v = Vector::new(f64::INFINITY, 0.0);
        assert_eq!(v.unwrap_err(), e);
        let v = Vector::new(f64::NEG_INFINITY, 0.0);
        assert_eq!(v.unwrap_err(), e);
        let v = Vector::new(0.0, f64::NAN);
        assert_eq!(v.unwrap_err(), e);
        let v = Vector::new(0.0, f64::INFINITY);
        assert_eq!(v.unwrap_err(), e);
        let v = Vector::new(0.0, f64::NEG_INFINITY);
        assert_eq!(v.unwrap_err(), e);
    }

    #[test]
    fn from_nalgebra() {
        let v = Vector::try_from(NaVec2::new(1.1, 2.2)).unwrap();
        rel_eq!(v.x(), 1.1);
        rel_eq!(v.y(), 2.2);
        let v = Vector::try_from(NaVec2::new(3.3, 4.4)).unwrap();
        rel_eq!(v.x(), 3.3);
        rel_eq!(v.y(), 4.4);
    }

    #[test]
    fn from_nalgebra_invalid_arguments() {
        let p = Vector::try_from(NaVec2::new(f64::NAN, 0.0));
        assert_eq!(p.unwrap_err(), RcgalError::NotFiniteInput);
        let p = Vector::try_from(NaVec2::new(f64::INFINITY, 0.0));
        assert_eq!(p.unwrap_err(), RcgalError::NotFiniteInput);
        let p = Vector::try_from(NaVec2::new(f64::NEG_INFINITY, 0.0));
        assert_eq!(p.unwrap_err(), RcgalError::NotFiniteInput);
        let p = Vector::try_from(NaVec2::new(0.0, f64::NAN));
        assert_eq!(p.unwrap_err(), RcgalError::NotFiniteInput);
        let p = Vector::try_from(NaVec2::new(0.0, f64::INFINITY));
        assert_eq!(p.unwrap_err(), RcgalError::NotFiniteInput);
        let p = Vector::try_from(NaVec2::new(0.0, f64::NEG_INFINITY));
        assert_eq!(p.unwrap_err(), RcgalError::NotFiniteInput);
    }

    #[test]
    fn to_nalgebra() {
        let p = Vector::new(1.1, 2.2).unwrap();
        let p = NaVec2::from(p);
        rel_eq!(p.x, 1.1);
        rel_eq!(p.y, 2.2);
        let p = Vector::new(3.3, 4.4).unwrap();
        let p = NaVec2::from(p);
        rel_eq!(p.x, 3.3);
        rel_eq!(p.y, 4.4);
    }

    #[test]
    fn vector_2d_norm_simple() {
        let v = Vector::new(0.0, 0.0).unwrap();
        rel_eq!(v.norm().unwrap(), 0.0);
        let v = Vector::new(3.0, 4.0).unwrap();
        rel_eq!(v.norm().unwrap(), 5.0);
        let v = Vector::new(1.0, 1.0).unwrap();
        rel_eq!(v.norm().unwrap(), C::SQRT_2);
    }

    #[test]
    fn vector_2d_norm_big() {
        let v = Vector::new(1e300, 0.0).unwrap();
        rel_eq!(v.norm().unwrap(), 1e300);
        let v = Vector::new(0.0, -2e300).unwrap();
        rel_eq!(v.norm().unwrap(), 2e300);
    }

    #[test]
    fn vector_2d_norm_small() {
        let v = Vector::new(1e-300, 0.0).unwrap();
        rel_eq!(v.norm().unwrap(), 1e-300);
        let v = Vector::new(0.0, -2e-300).unwrap();
        rel_eq!(v.norm().unwrap(), 2e-300);
    }

    #[test]
    fn vector_2d_norm_overflow() {
        let v = Vector::new(f64::MAX, f64::MAX).unwrap();
        assert_eq!(v.norm().unwrap_err(), RcgalError::Overflow);
    }
}
