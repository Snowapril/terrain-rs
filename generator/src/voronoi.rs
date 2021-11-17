use crate::utils::gen_rand_point;
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
        log::debug!("Create voronoi diagram with #{} vertices", points.len());
        let mut triangles: Vec<Triangle> = Vec::with_capacity(points.len() / 3);
        let super_triangle = Triangle::super_triangle(&points);
        triangles.push(super_triangle.clone());
        for point in points {
            let mut bad_triangles: Vec<Triangle> = Vec::new(); // TODO: need to use set?
            let mut edges: Vec<Edge> = Vec::new();
            for triangle in triangles.iter() {
                if triangle.check_circum_circle(&point) == Intersection::Inner {
                    edges.push(Edge::new(triangle.0, triangle.1));
                    edges.push(Edge::new(triangle.1, triangle.2));
                    edges.push(Edge::new(triangle.2, triangle.0));
                    bad_triangles.push(triangle.clone());
                }
            }
            // Remove duplication in edges
            edges.dedup();
            // Add triangulation
            for edge in edges {
                triangles.push(Triangle::new(edge.v1, edge.v2, point));
            }
            // Remove bad triangles from original triangles
            // TODO: Remove bad triangles from triangles
        }

        // Remove original super triangle
        triangles.remove(
            triangles
                .iter()
                .position(|triangle| triangle.eq(&super_triangle))
                .unwrap(),
        );

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
