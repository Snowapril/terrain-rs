use crate::vertex::{cross, signed_area, squared_length, Vertex};

/// triangle mesh topology constructed by three points
pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);

#[derive(PartialEq, Eq)]
pub enum Intersection {
    Inner,
    Overlapped,
    Outer,
}

impl Triangle {
    /// Return new triangle with given vertices
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex) -> Self {
        assert!(
            signed_area(&v1, &v2, &v3) < 0,
            "given vertices must construct triangle and ccw order"
        );
        Self(v1, v2, v3)
    }

    /// Return whether if given point is inside of circum circle of this triangle
    pub fn check_circum_circle(&self, point: &Vertex) -> Intersection {
        //! Implementation referenced on https://en.wikipedia.org/wiki/Circumscribed_circle
        let v1 = self.0 - *point;
        let v2 = self.1 - *point;
        let v3 = self.2 - *point;

        let threshold = squared_length(&v1) * cross(&v2, &v3)
            - squared_length(&v2) * cross(&v1, &v3)
            + squared_length(&v3) * cross(&v1, &v2);

        match threshold {
            val if val > 0 => Intersection::Inner,
            val if val < 0 => Intersection::Outer,
            _ => Intersection::Overlapped,
        }
    }

    pub fn super_triangle(_points: &[Vertex]) -> Self {
        todo!("create super triangle including all given points")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_circum_circle() {
        let triangle = Triangle::new(Vertex::new(-1, 0), Vertex::new(1, 0), Vertex::new(0, 2));
        assert!(triangle.check_circum_circle(&Vertex::new(0, -5)) == Intersection::Outer);
        assert!(triangle.check_circum_circle(&Vertex::new(0, 1)) == Intersection::Inner);
    }
}
