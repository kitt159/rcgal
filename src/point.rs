use crate::RcgalError;
use std::num::FpCategory;

/// Structure representing a 2D point.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    /// INVARIANT: x and y are finite
    inner: NaPoint2,
}

impl Point {
    /// Creates a new 2D point from x and y coordinates.
    ///
    /// # Errors
    /// Returns `NotFiniteInput` if the coordinates are infinite or NaN.
    pub fn new(x: f64, y: f64) -> Result<Self, RcgalError> {
        NaPoint2::new(x, y).try_into()
    }

    /// Returns the x coordinate.
    pub fn x(&self) -> f64 {
        self.inner.x
    }

    /// Returns the y coordinate.
    pub fn y(&self) -> f64 {
        self.inner.y
    }

    /// Returns the distance between two points.
    ///
    /// # Errors
    /// Returns `Overflow` if the distance is bigger than `f64::MAX`.
    pub fn dist(&self, other: &Self) -> Result<f64, RcgalError> {
        let diff = self.inner - other.inner;
        let dist = diff.x.hypot(diff.y);
        match dist.classify() {
            FpCategory::Zero | FpCategory::Normal | FpCategory::Subnormal => Ok(dist),
            FpCategory::Infinite => Err(RcgalError::Overflow),
            FpCategory::Nan => unreachable!("dist calculation should not return NaN"),
        }
    }
}

pub(crate) type NaPoint2 = nalgebra::Point2<f64>;

impl TryFrom<NaPoint2> for Point {
    type Error = RcgalError;
    fn try_from(value: NaPoint2) -> Result<Self, Self::Error> {
        (value.x.is_finite() && value.y.is_finite())
            .then_some(Self { inner: value })
            .ok_or(RcgalError::NotFiniteInput)
    }
}

impl From<Point> for NaPoint2 {
    fn from(value: Point) -> Self {
        value.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rel_eq;
    use core::f64;
    use std::f64::consts as C;

    #[test]
    fn point_2d_has_xy() {
        let p = Point::new(1.1, 2.2).unwrap();
        rel_eq!(p.x(), 1.1);
        rel_eq!(p.y(), 2.2);
        let p = Point::new(3.3, 4.4).unwrap();
        rel_eq!(p.x(), 3.3);
        rel_eq!(p.y(), 4.4);
    }

    #[test]
    fn point_2d_invalid_arguments() {
        let e = Point::new(f64::NAN, 0.0).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
        let e = Point::new(f64::INFINITY, 0.0).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
        let e = Point::new(f64::NEG_INFINITY, 0.0).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
        let e = Point::new(0.0, f64::NAN).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
        let e = Point::new(0.0, f64::INFINITY).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
        let e = Point::new(0.0, f64::NEG_INFINITY).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
    }

    #[test]
    fn from_nalgebra() {
        let p = Point::try_from(NaPoint2::new(1.1, 2.2)).unwrap();
        rel_eq!(p.x(), 1.1);
        rel_eq!(p.y(), 2.2);
        let p = Point::try_from(NaPoint2::new(3.3, 4.4)).unwrap();
        rel_eq!(p.x(), 3.3);
        rel_eq!(p.y(), 4.4);
    }

    #[test]
    fn from_nalgebra_invalid_arguments() {
        let e = Point::try_from(NaPoint2::new(f64::NAN, 0.0)).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
        let e = Point::try_from(NaPoint2::new(f64::INFINITY, 0.0)).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
        let e = Point::try_from(NaPoint2::new(f64::NEG_INFINITY, 0.0)).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
        let e = Point::try_from(NaPoint2::new(0.0, f64::NAN)).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
        let e = Point::try_from(NaPoint2::new(0.0, f64::INFINITY)).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
        let e = Point::try_from(NaPoint2::new(0.0, f64::NEG_INFINITY)).unwrap_err();
        assert_eq!(e, RcgalError::NotFiniteInput);
    }

    #[test]
    fn to_nalgebra() {
        let p = Point::new(1.1, 2.2).unwrap();
        let p = NaPoint2::from(p);
        rel_eq!(p.x, 1.1);
        rel_eq!(p.y, 2.2);
        let p = Point::new(3.3, 4.4).unwrap();
        let p = NaPoint2::from(p);
        rel_eq!(p.x, 3.3);
        rel_eq!(p.y, 4.4);
    }

    #[test]
    fn point_2d_to_point_2d_dist_simple() {
        let a = Point::new(0.0, 0.0).unwrap();
        let b = Point::new(1.0, 0.0).unwrap();
        rel_eq!(a.dist(&b).unwrap(), 1.0);
        let b = Point::new(3.0, 4.0).unwrap();
        rel_eq!(a.dist(&b).unwrap(), 5.0);
        let b = Point::new(1.0, 1.0).unwrap();
        rel_eq!(a.dist(&b).unwrap(), C::SQRT_2);
    }

    #[test]
    fn point_2d_to_point_2d_dist_comutativity() {
        let a = Point::new(12.34, 56.78).unwrap();
        let b = Point::new(87.65, 43.21).unwrap();
        rel_eq!(a.dist(&b).unwrap(), b.dist(&a).unwrap());
    }

    #[test]
    fn point_2d_to_point_2d_dist_big() {
        let a = Point::new(2e300, 0.0).unwrap();
        let b = Point::new(5e300, 0.0).unwrap();
        rel_eq!(a.dist(&b).unwrap(), 3e300);
    }

    #[test]
    fn point_2d_to_point_2d_dist_small() {
        let a = Point::new(2e-300, 0.0).unwrap();
        let b = Point::new(5e-300, 0.0).unwrap();
        rel_eq!(a.dist(&b).unwrap(), 3e-300);
    }

    #[test]
    fn point_2d_to_point_2d_dist_overflow() {
        let a = Point::new(f64::MAX, 0.0).unwrap();
        let b = Point::new(f64::MIN, 0.0).unwrap();
        assert_eq!(a.dist(&b).unwrap_err(), RcgalError::Overflow);
    }
}
