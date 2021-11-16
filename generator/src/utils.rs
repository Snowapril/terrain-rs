use cgmath::{point2, Point2};
use rand::{thread_rng, Rng};

pub fn gen_rand_point(min_corner: Point2<i32>, max_corner: Point2<i32>) -> Point2<i32> {
    let mut rng = thread_rng();
    let x = ((max_corner.x - min_corner.x) as f64 * rng.gen::<f64>()) as i32 + min_corner.x;
    let y = ((max_corner.y - min_corner.y) as f64 * rng.gen::<f64>()) as i32 + min_corner.y;
    point2(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rand_vertex() {
        let min_corner = point2(100, 100);
        let max_corner = point2(300, 300);

        for _ in 0..100 {
            let vertex = gen_rand_point(min_corner, max_corner);
            assert!(min_corner.x <= vertex.x && vertex.x <= max_corner.x);
            assert!(min_corner.y <= vertex.y && vertex.y <= max_corner.y);
        }
    }
}
