use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use forward_ref::{forward_ref_binop, forward_ref_op_assign};

use super::Parsable;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point2d {
    pub x: isize,
    pub y: isize,
}

impl Point2d {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    
    pub fn up(&mut self) -> &Self {
        self.y -= 1;
        self
    }

    pub fn down(&mut self) -> &Self {
        self.y += 1;
        self
    }

    pub fn left(&mut self) -> &Self {
        self.x -= 1;
        self
    }

    pub fn right(&mut self) -> &Self {
        self.x += 1;
        self
    }

    pub fn parse<T>(iter: &mut T) -> Self
    where
        T: IntoIterator<Item = u8> + Parsable<isize>
    {
        Point2d {
            x: iter.next_number().unwrap(),
            y: iter.next_number().unwrap(),
        }
    }
}

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
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

forward_ref_op_assign!(impl AddAssign, add_assign for Point2d, Point2d);

impl SubAssign for Point2d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

forward_ref_op_assign!(impl SubAssign, sub_assign for Point2d, Point2d);

impl MulAssign for Point2d {
    fn mul_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

forward_ref_op_assign!(impl MulAssign, mul_assign for Point2d, Point2d);


impl DivAssign for Point2d {
    fn div_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

forward_ref_op_assign!(impl DivAssign, div_assign for Point2d, Point2d);