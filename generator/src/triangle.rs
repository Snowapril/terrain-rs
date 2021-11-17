use crate::cga::{cross, signed_area, squared_length};
use cgmath::{point2, Point2};

/// Triangle edge
#[derive(PartialEq, Eq)]
pub struct Edge {
    pub v1: Point2<i32>,
    pub v2: Point2<i32>,
}

impl Edge {
    pub fn new(v1: Point2<i32>, v2: Point2<i32>) -> Self {
        Self { v1, v2 }
    }
}

/// triangle mesh topology constructed by three points
#[derive(Clone, PartialEq, Eq)]
pub struct Triangle(pub Point2<i32>, pub Point2<i32>, pub Point2<i32>);

#[derive(PartialEq, Eq)]
pub enum Intersection {
    Inner,
    Overlapped,
    Outer,
}

impl Triangle {
    /// Return new triangle with given vertices
    pub fn new(v1: Point2<i32>, v2: Point2<i32>, v3: Point2<i32>) -> Self {
        // assert!(
        //     signed_area(&v1, &v2, &v3) < 0,
        //     "given vertices must construct triangle and ccw order"
        // );
        Self(v1, v2, v3)
    }

    /// Return whether if given point is inside of circum circle of this triangle
    pub fn check_circum_circle(&self, point: &Point2<i32>) -> Intersection {
        //! Implementation referenced on https://en.wikipedia.org/wiki/Circumscribed_circle
        let v1 = self.0 - *point;
        let v2 = self.1 - *point;
        let v3 = self.2 - *point;

        let threshold: isize = (squared_length(&v1) * cross(&v2, &v3)
            - squared_length(&v2) * cross(&v1, &v3)
            + squared_length(&v3) * cross(&v1, &v2)) as isize;

        match threshold {
            val if val > 0 => Intersection::Inner,
            val if val < 0 => Intersection::Outer,
            _ => Intersection::Overlapped,
        }
    }

    pub fn super_triangle(points: &[Point2<i32>]) -> Self {
        // TODO: make more correct super triangle
        let min_corner = points
            .iter()
            .fold(point2(i32::MAX, i32::MAX), |min_corner, vertex| {
                point2(
                    i32::min(min_corner.x, vertex.x),
                    i32::min(min_corner.y, vertex.y),
                )
            });
        let max_corner = points
            .iter()
            .fold(point2(i32::MIN, i32::MIN), |max_corner, vertex| {
                point2(
                    i32::max(max_corner.x, vertex.x),
                    i32::max(max_corner.y, vertex.y),
                )
            });
        let scale = max_corner - min_corner;
        Self(
            point2(min_corner.x - scale.x, min_corner.y - scale.y),
            point2(min_corner.x - scale.x, max_corner.y + scale.y),
            point2(max_corner.x + scale.x, scale.y / 2),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_circum_circle() {
        let triangle = Triangle::new(point2(-1, 0), point2(1, 0), point2(0, 2));
        assert!(triangle.check_circum_circle(&point2(0, -5)) == Intersection::Outer);
        assert!(triangle.check_circum_circle(&point2(0, 1)) == Intersection::Inner);
    }
}
