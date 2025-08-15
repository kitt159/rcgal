use crate::{Point, Vector};
use std::ops::{Add, Sub};

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rel_eq;

    #[test]
    fn point_2d_sub_point_2d() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(3.0, 4.0);
        let c = b - a;
        rel_eq!(c.x(), 2.0);
        rel_eq!(c.y(), 2.0);
        let c = a - b;
        rel_eq!(c.x(), -2.0);
        rel_eq!(c.y(), -2.0);
    }

    #[test]
    fn point_2d_add_vector_2d() {
        let a = Point::new(1.0, 2.0);
        let b = Vector::new(-1.0, -2.0);
        let c = a + b;
        rel_eq!(c.x(), 0.0);
        rel_eq!(c.y(), 0.0);
        let b = Vector::new(3.0, 4.0);
        let c = a + b;
        rel_eq!(c.x(), 4.0);
        rel_eq!(c.y(), 6.0);
    }
}
