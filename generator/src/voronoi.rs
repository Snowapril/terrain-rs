use crate::{Edge, Intersection, Triangle};
use cgmath::Point2;
#[allow(dead_code)]
pub struct Voronoi {
    triangles: Vec<Triangle>,
}

impl Voronoi {
    /// Generate voronoi diagram using BowyerWatson algorithm
    /// refernced by https://en.wikipedia.org/wiki/Bowyer%E2%80%93Watson_algorithm
    pub fn new(points: Vec<Point2<i32>>) -> Self {
        let mut triangles: Vec<Triangle> = Vec::with_capacity(points.len() / 3);
        triangles.push(Triangle::super_triangle(&points));
        for point in points {
            let mut _bad_triangles: Vec<Triangle> = Vec::new(); // TODO: need to use set?
            for triangle in &triangles {
                if triangle.check_circum_circle(&point) == Intersection::Inner {
                    // bad_triangles.push(triangle.to_owned());
                }
            }
            let mut _polygon: Vec<Triangle> = Vec::new();
            todo!("implement rest algorithms")
        }

        Self { triangles }
    }

    pub fn random_voronoi(
        min_corner: Point2<i32>,
        max_corner: Point2<i32>,
        num_points: usize,
    ) -> Self {
        let mut points: Vec<Point2<i32>> = Vec::with_capacity(num_points);
        for _ in 0..num_points {
            points.push(gen_rand_point(min_corner, max_corner));
        }

        Self::new(points)
    }
}
