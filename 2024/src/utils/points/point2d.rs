use crate::utils::{grid::iterators::ADJ_FOUR, Parsable};

#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Point2d {
    pub x: isize,
    pub y: isize,
}

impl Point2d {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn move_up(&mut self) -> &Self {
        self.y -= 1;
        self
    }

    pub fn move_down(&mut self) -> &Self {
        self.y += 1;
        self
    }

    pub fn move_left(&mut self) -> &Self {
        self.x -= 1;
        self
    }

    pub fn move_right(&mut self) -> &Self {
        self.x += 1;
        self
    }

    pub fn move_dir4(&mut self, dir: usize) -> &Self {
        let d = ADJ_FOUR[dir];
        self.x += d.0;
        self.y += d.1;
        self
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn dir4(&self, dir: usize) -> Self {
        let d = ADJ_FOUR[dir];
        Self {
            x: self.x + d.0,
            y: self.y + d.1,
        }
    }

    pub fn determinant(&self, other: &Self) -> isize {
        self.x * other.y - other.x * self.y
    }

    pub fn l1(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }

    pub fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    pub fn parse<T>(iter: &mut T) -> Self
    where
        T: IntoIterator<Item = u8> + Parsable<isize>,
    {
        Self {
            x: iter.next_number().unwrap(),
            y: iter.next_number().unwrap(),
        }
    }

    pub fn try_parse<T>(iter: &mut T) -> Option<Self>
    where
        T: IntoIterator<Item = u8> + Parsable<isize>,
    {
        if let Some((x, y)) = iter.next_number().zip(iter.next_number()) {
            return Some(Self { x, y });
        }
        None
    }
}
