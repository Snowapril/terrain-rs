use crate::common::Point2;
use rand::{thread_rng, Rng};

pub type Vertex = Point2<i32>;

pub fn gen_rand_vertex(min_corner: Vertex, max_corner: Vertex) -> Vertex {
    let mut rng = thread_rng();
    let x: i32 = ((max_corner.x - min_corner.x) as f64 * rng.gen::<f64>()) as i32 + min_corner.x;
    let y: i32 = ((max_corner.y - min_corner.y) as f64 * rng.gen::<f64>()) as i32 + min_corner.y;
    Vertex { x, y }
}

pub fn signed_area(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> i32 {
    v2.x * v1.y + v3.x * v2.y + v1.x * v3.y - v1.x * v2.y - v2.x * v3.y - v3.x * v1.y
}

pub fn squared_length(v: &Vertex) -> i32 {
    v.x * v.x + v.y * v.y
}

pub fn cross(v1: &Vertex, v2: &Vertex) -> i32 {
    v1.x * v2.y - v2.x * v1.y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rand_vertex() {
        let min_corner = Vertex::new(100, 100);
        let max_corner = Vertex::new(300, 300);

        for _ in 0..100 {
            let vertex = gen_rand_vertex(min_corner, max_corner);
            assert!(min_corner.x <= vertex.x && vertex.x <= max_corner.x);
            assert!(min_corner.y <= vertex.y && vertex.y <= max_corner.y);
        }
    }

    #[test]
    fn test_signed_area() {
        let v1 = Vertex::new(-3, -4);
        let v2 = Vertex::new(1, 3);
        let v3 = Vertex::new(3, -2);

        assert!(signed_area(&v1, &v2, &v3) > 0);
        assert!(signed_area(&v1, &v3, &v2) < 0);
    }

    #[test]
    fn test_squared_length() {
        let v = Vertex::new(3, 4);
        assert!(squared_length(&v) == 25);
    }

    #[test]
    fn test_cross() {
        let v1 = Vertex::new(3, 4);
        let v2 = Vertex::new(5, 1);
        assert!(cross(&v1, &v2) == -17);
    }
}
