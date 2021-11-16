use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! point_impl {
    ($t:ident {$($field:ident), +}) => {
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
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Point4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

point_impl!(Point2 { x, y });
point_impl!(Point3 { x, y, z });
point_impl!(Point4 { x, y, z, w });

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_vec() {
        use super::*;
        assert_eq!(
            Point2::<i32>::new(1, 2) + Point2::<i32>::new(3, 4),
            Point2::<i32>::new(4, 6)
        );

        assert_eq!(
            Point3::<i32>::new(1, 2, 3) + Point3::<i32>::new(4, 5, 6),
            Point3::<i32>::new(5, 7, 9)
        );

        assert_eq!(
            Point4::<i32>::new(1, 2, 3, 4) + Point4::<i32>::new(-1, -2, -3, -4),
            Point4::<i32>::new(0, 0, 0, 0)
        );
    }

    #[test]
    fn test_add_assign_vec() {
        use super::*;
        let mut point2 = Point2::<i32>::new(1, 2);
        point2 += Point2::<i32>::new(3, 4);
        assert_eq!(point2, Point2::<i32>::new(4, 6));

        let mut point3 = Point3::<i32>::new(1, 2, 3);
        point3 += Point3::<i32>::new(4, 5, 6);
        assert_eq!(point3, Point3::<i32>::new(5, 7, 9));

        let mut point4 = Point4::<i32>::new(1, 2, 3, 4);
        point4 += Point4::<i32>::new(-1, -2, -3, -4);
        assert_eq!(point4, Point4::<i32>::new(0, 0, 0, 0));
    }

    #[test]
    fn test_sub_vec() {
        use super::*;
        assert_eq!(
            Point2::<i32>::new(1, 2) - Point2::<i32>::new(3, 4),
            Point2::<i32>::new(-2, -2)
        );

        assert_eq!(
            Point3::<i32>::new(1, 2, 3) - Point3::<i32>::new(4, 5, 6),
            Point3::<i32>::new(-3, -3, -3)
        );

        assert_eq!(
            Point4::<i32>::new(1, 2, 3, 4) - Point4::<i32>::new(-1, -2, -3, -4),
            Point4::<i32>::new(2, 4, 6, 8)
        );
    }

    #[test]
    fn test_sub_assign_vec() {
        use super::*;
        let mut point2 = Point2::<i32>::new(1, 2);
        point2 -= Point2::<i32>::new(3, 4);
        assert_eq!(point2, Point2::<i32>::new(-2, -2));

        let mut point3 = Point3::<i32>::new(1, 2, 3);
        point3 -= Point3::<i32>::new(4, 5, 6);
        assert_eq!(point3, Point3::<i32>::new(-3, -3, -3));

        let mut point4 = Point4::<i32>::new(1, 2, 3, 4);
        point4 -= Point4::<i32>::new(-1, -2, -3, -4);
        assert_eq!(point4, Point4::<i32>::new(2, 4, 6, 8));
    }

    #[test]
    fn test_mul_vec() {
        use super::*;
        assert_eq!(
            Point2::<i32>::new(1, 2) * Point2::<i32>::new(3, 4),
            Point2::<i32>::new(3, 8)
        );

        assert_eq!(
            Point3::<i32>::new(1, 2, 3) * Point3::<i32>::new(4, 5, 6),
            Point3::<i32>::new(4, 10, 18)
        );

        assert_eq!(
            Point4::<i32>::new(1, 2, 3, 4) * Point4::<i32>::new(-1, -2, -3, -4),
            Point4::<i32>::new(-1, -4, -9, -16)
        );
    }

    #[test]
    fn test_mul_assign_vec() {
        use super::*;
        let mut point2 = Point2::<i32>::new(1, 2);
        point2 *= Point2::<i32>::new(3, 4);
        assert_eq!(point2, Point2::<i32>::new(3, 8));

        let mut point3 = Point3::<i32>::new(1, 2, 3);
        point3 *= Point3::<i32>::new(4, 5, 6);
        assert_eq!(point3, Point3::<i32>::new(4, 10, 18));

        let mut point4 = Point4::<i32>::new(1, 2, 3, 4);
        point4 *= Point4::<i32>::new(-1, -2, -3, -4);
        assert_eq!(point4, Point4::<i32>::new(-1, -4, -9, -16));
    }

    #[test]
    fn test_div_vec() {
        use super::*;
        assert_eq!(
            Point2::<i32>::new(2, 6) / Point2::<i32>::new(2, 3),
            Point2::<i32>::new(1, 2)
        );

        assert_eq!(
            Point3::<i32>::new(-3, 2, 4) / Point3::<i32>::new(-1, 2, 2),
            Point3::<i32>::new(3, 1, 2)
        );

        assert_eq!(
            Point4::<i32>::new(1, 2, 3, 4) / Point4::<i32>::new(-1, -2, -3, -4),
            Point4::<i32>::new(-1, -1, -1, -1)
        );
    }

    #[test]
    fn test_div_assign_vec() {
        use super::*;
        let mut point2 = Point2::<i32>::new(2, 6);
        point2 /= Point2::<i32>::new(2, 3);
        assert_eq!(point2, Point2::<i32>::new(1, 2));

        let mut point3 = Point3::<i32>::new(-3, 2, 4);
        point3 /= Point3::<i32>::new(-1, 2, 2);
        assert_eq!(point3, Point3::<i32>::new(3, 1, 2));

        let mut point4 = Point4::<i32>::new(1, 2, 3, 4);
        point4 /= Point4::<i32>::new(-1, -2, -3, -4);
        assert_eq!(point4, Point4::<i32>::new(-1, -1, -1, -1));
    }

    #[test]
    fn test_neg_vec() {
        use super::*;
        assert_eq!(-Point2::<i32>::new(1, 2), Point2::<i32>::new(-1, -2));

        assert_eq!(-Point3::<i32>::new(1, 2, 3), Point3::<i32>::new(-1, -2, -3));

        assert_eq!(
            -Point4::<i32>::new(1, 2, 3, 4),
            Point4::<i32>::new(-1, -2, -3, -4)
        );
    }
}
