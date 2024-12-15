use crate::utils::{grid::iterators::ADJ_FOUR, Parsable};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point2d {
    pub x: isize,
    pub y: isize,
}

impl Point2d {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    
    pub fn move_dir(&self, dir: usize) -> Self {
        let d = ADJ_FOUR[dir];
        Self {
            x: self.x + d.0,
            y: self.y + d.1,
        }
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

    pub fn determinant(&self, other: &Self) -> isize {
        self.x * other.y - other.x * self.y
    }

    pub fn parse<T>(iter: &mut T) -> Self
    where
        T: IntoIterator<Item = u8> + Parsable<isize>
    {
        Self {
            x: iter.next_number().unwrap(),
            y: iter.next_number().unwrap(),
        }
    }
}
