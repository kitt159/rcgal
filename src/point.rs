type NaPoint2 = nalgebra::Point2<f64>;

/// Structure representing a 2D point.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    inner: NaPoint2,
}

impl Point {
    /// Creates a new 2D point from x and y coordinates.
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            inner: NaPoint2::new(x, y),
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

    /// Returns the distance between two points.
    pub fn dist(&self, other: &Self) -> f64 {
        let diff = self.inner - other.inner;
        diff.x.hypot(diff.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rel_eq;
    use std::f64::consts as C;

    #[test]
    fn point_2d_has_xy() {
        let p = Point::new(1.1, 2.2);
        rel_eq!(p.x(), 1.1);
        rel_eq!(p.y(), 2.2);
        let p = Point::new(3.3, 4.4);
        rel_eq!(p.x(), 3.3);
        rel_eq!(p.y(), 4.4);
    }

    #[test]
    fn point_2d_to_point_2d_dist_simple() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
        rel_eq!(a.dist(&b), 1.0);
        let b = Point::new(3.0, 4.0);
        rel_eq!(a.dist(&b), 5.0);
        let b = Point::new(1.0, 1.0);
        rel_eq!(a.dist(&b), C::SQRT_2);
    }

    #[test]
    fn point_2d_to_point_2d_dist_comutativity() {
        let a = Point::new(12.34, 56.78);
        let b = Point::new(87.65, 43.21);
        rel_eq!(a.dist(&b), b.dist(&a));
    }

    #[test]
    fn point_2d_to_point_2d_dist_big() {
        let a = Point::new(2e300, 0.0);
        let b = Point::new(5e300, 0.0);
        rel_eq!(a.dist(&b), 3e300);
    }

    #[test]
    fn point_2d_to_point_2d_dist_small() {
        let a = Point::new(2e-300, 0.0);
        let b = Point::new(5e-300, 0.0);
        rel_eq!(a.dist(&b), 3e-300);
    }

    #[test]
    fn point_2d_to_point_2d_dist_overflow() {
        let a = Point::new(f64::MAX, 0.0);
        let b = Point::new(f64::MIN, 0.0);
        assert!(a.dist(&b).is_infinite());
    }

    #[test]
    fn point_2d_to_point_2d_dist_invalid_arguments() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(f64::INFINITY, 0.0);
        assert!(a.dist(&b).is_infinite());
        let b = Point::new(f64::NEG_INFINITY, 0.0);
        assert!(a.dist(&b).is_infinite());
        let b = Point::new(f64::NAN, 0.0);
        assert!(a.dist(&b).is_nan());
        let a = Point::new(f64::INFINITY, 0.0);
        let b = Point::new(f64::INFINITY, 0.0);
        assert!(a.dist(&b).is_nan());
        let b = Point::new(f64::NEG_INFINITY, 0.0);
        assert!(a.dist(&b).is_infinite());
        let b = Point::new(0.0, f64::INFINITY);
        assert!(a.dist(&b).is_infinite());
        let b = Point::new(0.0, f64::NEG_INFINITY);
        assert!(a.dist(&b).is_infinite());
        let b = Point::new(0.0, f64::NAN);
        assert!(a.dist(&b).is_infinite());
    }
}
