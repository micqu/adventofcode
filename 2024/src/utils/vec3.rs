use std::{iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use forward_ref::{forward_ref_binop, forward_ref_op_assign};

use super::Parsable;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - other.y * self.z,
            y: self.z * other.x - other.z * self.x,
            z: self.x * other.y - other.x * self.y,
        }
    }

    pub fn parse<T>(iter: &mut T) -> Self
    where
        T: IntoIterator<Item = u8> + Parsable<f64>,
    {
        Self {
            x: iter.next_number().unwrap(),
            y: iter.next_number().unwrap(),
            z: iter.next_number().unwrap(),
        }
    }
}


impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

forward_ref_binop!(impl Add, add for Vec3, Vec3);

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

forward_ref_binop!(impl Sub, sub for Vec3, Vec3);

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

forward_ref_binop!(impl Mul, mul for Vec3, f64);

impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

forward_ref_binop!(impl Div, div for Vec3, f64);

impl Sub<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

forward_ref_binop!(impl Sub, sub for Vec3, f64);


impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

forward_ref_op_assign!(impl AddAssign, add_assign for Vec3, Vec3);

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

forward_ref_op_assign!(impl SubAssign, sub_assign for Vec3, Vec3);

impl MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

forward_ref_op_assign!(impl MulAssign, mul_assign for Vec3, Vec3);


impl DivAssign for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

forward_ref_op_assign!(impl DivAssign, div_assign for Vec3, Vec3);

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let (mut x, mut y, mut z) = (0f64, 0f64, 0f64);
        for p in iter {
            x += p.x;
            y += p.y;
            z += p.z;
        }
        Vec3::new(x, y, z)
    }
}
