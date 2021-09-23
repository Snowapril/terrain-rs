use rand::{thread_rng, Rng};
use terrain_common::point::Point2;

pub type Vertex = Point2<i32>;

fn gen_rand_vertex(min_corner: Vertex, max_corner: Vertex) -> Vertex {
    let mut rng = thread_rng();
    let x: i32 = ((max_corner.x - min_corner.x) as f64 * rng.gen::<f64>()) as i32 + min_corner.x;
    let y: i32 = ((max_corner.y - min_corner.y) as f64 * rng.gen::<f64>()) as i32 + min_corner.y;
    Vertex { x, y }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rand_vertex() {
        use super::*;
        let min_corner = Vertex::new(100, 100);
        let max_corner = Vertex::new(300, 300);

        for _ in 0..100 {
            let vertex = gen_rand_vertex(min_corner, max_corner);
            assert!(min_corner.x <= vertex.x && vertex.x <= max_corner.x);
            assert!(min_corner.y <= vertex.y && vertex.y <= max_corner.y);
        }
    }
}
