use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! vec_impl {
    ($t:ident {$($field:ident), +}, $n:expr) => {
        impl<T> $t<T> {
            #[inline]
            pub fn new($($field: T), +)-> $t<T> {
                $t { $($field), + }
            }
        }

        impl<T> Add for $t<T>
        where T: Add<Output = T> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self {
                    $($field: self.$field + rhs.$field), +
                }
            }
        }

        impl<T> AddAssign for $t<T>
        where T: AddAssign {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$field += rhs.$field);+
            }
        }

        impl<T> Sub for $t<T>
        where T: Sub<Output = T> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self {
                    $($field: self.$field - rhs.$field), +
                }
            }
        }

        impl<T> SubAssign for $t<T>
        where T: SubAssign {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$field -= rhs.$field);+
            }
        }

        impl<T> Div for $t<T>
        where T: Div<Output = T> {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                Self {
                    $($field: self.$field / rhs.$field), +
                }
            }
        }

        impl<T> DivAssign for $t<T>
        where T: DivAssign {
            fn div_assign(&mut self, rhs: Self) {
                $(self.$field /= rhs.$field);+
            }
        }

        impl<T> Mul for $t<T>
        where T: Mul<Output = T> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self {
                    $($field: self.$field * rhs.$field), +
                }
            }
        }

        impl<T> MulAssign for $t<T>
        where T: MulAssign {
            fn mul_assign(&mut self, rhs: Self) {
                $(self.$field *= rhs.$field);+
            }
        }

        impl<T> Neg for $t<T>
        where T: Neg<Output = T> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self {
                    $($field: -self.$field), +
                }
            }
        }
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

vec_impl!(Vec2 { x, y }, 2);
vec_impl!(Vec3 { x, y, z }, 3);
vec_impl!(Vec4 { x, y, z, w }, 4);

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_vec() {
        use super::*;
        assert_eq!(
            Vec2::<i32>::new(1, 2) + Vec2::<i32>::new(3, 4),
            Vec2::<i32>::new(4, 6)
        );

        assert_eq!(
            Vec3::<i32>::new(1, 2, 3) + Vec3::<i32>::new(4, 5, 6),
            Vec3::<i32>::new(5, 7, 9)
        );

        assert_eq!(
            Vec4::<i32>::new(1, 2, 3, 4) + Vec4::<i32>::new(-1, -2, -3, -4),
            Vec4::<i32>::new(0, 0, 0, 0)
        );
    }

    #[test]
    fn test_add_assign_vec() {
        use super::*;
        let mut vec2 = Vec2::<i32>::new(1, 2);
        vec2 += Vec2::<i32>::new(3, 4);
        assert_eq!(vec2, Vec2::<i32>::new(4, 6));

        let mut vec3 = Vec3::<i32>::new(1, 2, 3);
        vec3 += Vec3::<i32>::new(4, 5, 6);
        assert_eq!(vec3, Vec3::<i32>::new(5, 7, 9));

        let mut vec4 = Vec4::<i32>::new(1, 2, 3, 4);
        vec4 += Vec4::<i32>::new(-1, -2, -3, -4);
        assert_eq!(vec4, Vec4::<i32>::new(0, 0, 0, 0));
    }

    #[test]
    fn test_sub_vec() {
        use super::*;
        assert_eq!(
            Vec2::<i32>::new(1, 2) - Vec2::<i32>::new(3, 4),
            Vec2::<i32>::new(-2, -2)
        );

        assert_eq!(
            Vec3::<i32>::new(1, 2, 3) - Vec3::<i32>::new(4, 5, 6),
            Vec3::<i32>::new(-3, -3, -3)
        );

        assert_eq!(
            Vec4::<i32>::new(1, 2, 3, 4) - Vec4::<i32>::new(-1, -2, -3, -4),
            Vec4::<i32>::new(2, 4, 6, 8)
        );
    }

    #[test]
    fn test_sub_assign_vec() {
        use super::*;
        let mut vec2 = Vec2::<i32>::new(1, 2);
        vec2 -= Vec2::<i32>::new(3, 4);
        assert_eq!(vec2, Vec2::<i32>::new(-2, -2));

        let mut vec3 = Vec3::<i32>::new(1, 2, 3);
        vec3 -= Vec3::<i32>::new(4, 5, 6);
        assert_eq!(vec3, Vec3::<i32>::new(-3, -3, -3));

        let mut vec4 = Vec4::<i32>::new(1, 2, 3, 4);
        vec4 -= Vec4::<i32>::new(-1, -2, -3, -4);
        assert_eq!(vec4, Vec4::<i32>::new(2, 4, 6, 8));
    }

    #[test]
    fn test_mul_vec() {
        use super::*;
        assert_eq!(
            Vec2::<i32>::new(1, 2) * Vec2::<i32>::new(3, 4),
            Vec2::<i32>::new(3, 8)
        );

        assert_eq!(
            Vec3::<i32>::new(1, 2, 3) * Vec3::<i32>::new(4, 5, 6),
            Vec3::<i32>::new(4, 10, 18)
        );

        assert_eq!(
            Vec4::<i32>::new(1, 2, 3, 4) * Vec4::<i32>::new(-1, -2, -3, -4),
            Vec4::<i32>::new(-1, -4, -9, -16)
        );
    }

    #[test]
    fn test_mul_assign_vec() {
        use super::*;
        let mut vec2 = Vec2::<i32>::new(1, 2);
        vec2 *= Vec2::<i32>::new(3, 4);
        assert_eq!(vec2, Vec2::<i32>::new(3, 8));

        let mut vec3 = Vec3::<i32>::new(1, 2, 3);
        vec3 *= Vec3::<i32>::new(4, 5, 6);
        assert_eq!(vec3, Vec3::<i32>::new(4, 10, 18));

        let mut vec4 = Vec4::<i32>::new(1, 2, 3, 4);
        vec4 *= Vec4::<i32>::new(-1, -2, -3, -4);
        assert_eq!(vec4, Vec4::<i32>::new(-1, -4, -9, -16));
    }

    #[test]
    fn test_div_vec() {
        use super::*;
        assert_eq!(
            Vec2::<i32>::new(2, 6) / Vec2::<i32>::new(2, 3),
            Vec2::<i32>::new(1, 2)
        );

        assert_eq!(
            Vec3::<i32>::new(-3, 2, 4) / Vec3::<i32>::new(-1, 2, 2),
            Vec3::<i32>::new(3, 1, 2)
        );

        assert_eq!(
            Vec4::<i32>::new(1, 2, 3, 4) / Vec4::<i32>::new(-1, -2, -3, -4),
            Vec4::<i32>::new(-1, -1, -1, -1)
        );
    }

    #[test]
    fn test_div_assign_vec() {
        use super::*;
        let mut vec2 = Vec2::<i32>::new(2, 6);
        vec2 /= Vec2::<i32>::new(2, 3);
        assert_eq!(vec2, Vec2::<i32>::new(1, 2));

        let mut vec3 = Vec3::<i32>::new(-3, 2, 4);
        vec3 /= Vec3::<i32>::new(-1, 2, 2);
        assert_eq!(vec3, Vec3::<i32>::new(3, 1, 2));

        let mut vec4 = Vec4::<i32>::new(1, 2, 3, 4);
        vec4 /= Vec4::<i32>::new(-1, -2, -3, -4);
        assert_eq!(vec4, Vec4::<i32>::new(-1, -1, -1, -1));
    }

    #[test]
    fn test_neg_vec() {
        use super::*;
        assert_eq!(-Vec2::<i32>::new(1, 2), Vec2::<i32>::new(-1, -2));

        assert_eq!(-Vec3::<i32>::new(1, 2, 3), Vec3::<i32>::new(-1, -2, -3));

        assert_eq!(
            -Vec4::<i32>::new(1, 2, 3, 4),
            Vec4::<i32>::new(-1, -2, -3, -4)
        );
    }
}
