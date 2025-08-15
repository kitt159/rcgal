/// Structure representing a 2D vector.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    /// Creates a new 2D vector from x and y coordinates.
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x, y }
    }

    /// Returns the x coordinate.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the y coordinate.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns the length of the vector.
    pub fn length(&self) -> f64 {
        self.x.hypot(self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rel_eq;

    #[test]
    fn vector_2d_has_xy() {
        let p = Vector::new(1.1, 2.2);
        rel_eq!(p.x(), 1.1);
        rel_eq!(p.y(), 2.2);
        let p = Vector::new(3.3, 4.4);
        rel_eq!(p.x(), 3.3);
        rel_eq!(p.y(), 4.4);
    }

    #[test]
    fn vector_2d_length() {
        let p = Vector::new(3.0, 4.0);
        rel_eq!(p.length(), 5.0);
    }
}
