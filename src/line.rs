use crate::{Point, RcgalError, Vector};

/// Structure representing a line in 2D space.
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    location: Point,
    /// INVARIANT: unit vector
    direction: Vector,
}

impl Line {
    /// Creates a new 2D line from a location and a direction.
    ///
    /// # Errors
    /// Returns `InvalidInput` if all components of direction vector are subnormal.
    ///
    /// Returns `NotFiniteInput` If the location or direction is not rcgal::Point or rcgal::Vector, respectively,
    /// and the coordinates are infinite or NaN.
    pub fn new<P, V>(location: P, direction: V) -> Result<Self, RcgalError>
    where
        P: TryInto<Point>,
        RcgalError: From<P::Error>,
        V: TryInto<Vector>,
        RcgalError: From<V::Error>,
    {
        Ok(Self {
            location: location.try_into()?,
            direction: direction.try_into()?.normalize()?,
        })
    }

    /// Returns the anchor point through which the line passes.
    pub fn location(&self) -> Point {
        self.location
    }

    /// Returns the direction of the line.
    ///
    /// Always returns a unit vector.
    pub fn direction(&self) -> Vector {
        self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rel_eq;

    #[test]
    fn line_has_location_and_direction() {
        let l = Line::new(
            Point::new(1.0, 2.0).unwrap(),
            Vector::new(1.0, 0.0).unwrap(),
        )
        .unwrap();
        rel_eq!(l.location().x(), 1.0);
        rel_eq!(l.location().y(), 2.0);
        rel_eq!(l.direction().x(), 1.0);
        rel_eq!(l.direction().y(), 0.0);
        let l = Line::new(
            Point::new(5.0, 6.0).unwrap(),
            Vector::new(3.0, 4.0).unwrap(),
        )
        .unwrap();
        rel_eq!(l.location().x(), 5.0);
        rel_eq!(l.location().y(), 6.0);
        rel_eq!(l.direction().x(), 3.0 / 5.0);
        rel_eq!(l.direction().y(), 4.0 / 5.0);
    }
}
