use cgmath::{Point2, Vector2};

pub fn signed_area(v1: &Point2<i32>, v2: &Point2<i32>, v3: &Point2<i32>) -> i32 {
    v2.x * v1.y + v3.x * v2.y + v1.x * v3.y - v1.x * v2.y - v2.x * v3.y - v3.x * v1.y
}

pub fn squared_length(v: &Vector2<i32>) -> i32 {
    v.x * v.x + v.y * v.y
}

pub fn cross(v1: &Vector2<i32>, v2: &Vector2<i32>) -> i32 {
    v1.x * v2.y - v2.x * v1.y
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::{point2, vec2};
    #[test]
    fn test_signed_area() {
        let v1 = point2(-3, -4);
        let v2 = point2(1, 3);
        let v3 = point2(3, -2);

        assert!(signed_area(&v1, &v2, &v3) > 0);
        assert!(signed_area(&v1, &v3, &v2) < 0);
    }

    #[test]
    fn test_squared_length() {
        let v = vec2(3, 4);
        assert!(squared_length(&v) == 25);
    }

    #[test]
    fn test_cross() {
        let v1 = vec2(3, 4);
        let v2 = vec2(5, 1);
        assert!(cross(&v1, &v2) == -17);
    }
}
