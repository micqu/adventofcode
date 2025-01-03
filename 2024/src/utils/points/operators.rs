use std::{iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use forward_ref::{forward_ref_binop, forward_ref_op_assign};

use super::point2d::Point2d;

impl Add for Point2d {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

forward_ref_binop!(impl Add, add for Point2d, Point2d);

impl Sub for Point2d {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

forward_ref_binop!(impl Sub, sub for Point2d, Point2d);

impl Mul<isize> for Point2d {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: isize) -> Self::Output {
        Point2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

forward_ref_binop!(impl Mul, mul for Point2d, isize);

impl Div<isize> for Point2d {
    type Output = Self;

    #[inline]
    fn div(self, rhs: isize) -> Self::Output {
        Point2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

forward_ref_binop!(impl Div, div for Point2d, isize);

impl Sub<isize> for Point2d {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: isize) -> Self::Output {
        Point2d {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

forward_ref_binop!(impl Sub, sub for Point2d, isize);


impl AddAssign for Point2d {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

forward_ref_op_assign!(impl AddAssign, add_assign for Point2d, Point2d);

impl SubAssign for Point2d {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

forward_ref_op_assign!(impl SubAssign, sub_assign for Point2d, Point2d);

impl MulAssign for Point2d {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

forward_ref_op_assign!(impl MulAssign, mul_assign for Point2d, Point2d);


impl DivAssign for Point2d {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

forward_ref_op_assign!(impl DivAssign, div_assign for Point2d, Point2d);

impl Sum for Point2d {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let (mut x, mut y) = (0, 0);
        for p in iter {
            x += p.x;
            y += p.y;
        }
        Point2d::new(x, y)
    }
}

// impl AddAssign<(isize, isize)> for Point2d {
//     fn add_assign(&mut self, rhs: (isize, isize)) {
//         self.x += rhs.0;
//         self.y += rhs.1;
//     }
// }

// forward_ref_op_assign!(impl AddAssign, add_assign for Point2d, (isize, isize));