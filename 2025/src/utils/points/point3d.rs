use num::integer::Roots;

use crate::utils::{Parsable, grid::iterators::ADJ_FOUR};

#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Point3d<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Point3d<isize> {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn l2(&self, other: &Self) -> isize {
        ((self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z))
            .sqrt()
    }

    pub fn parse<T>(iter: &mut T) -> Self
    where
        T: IntoIterator<Item = u8> + Parsable<isize>,
    {
        Self {
            x: iter.next_number().unwrap(),
            y: iter.next_number().unwrap(),
            z: iter.next_number().unwrap(),
        }
    }
}
