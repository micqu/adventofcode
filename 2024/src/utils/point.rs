use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn div_n(&self, n: isize) -> Self {
        Self {
            x: self.x / n,
            y: self.y / n,
        }
    }

    pub fn mul_n(&self, n: isize) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
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
}

impl Sub for &Point {
    type Output = Point;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<isize> for Point {
    type Output = Point;
    
    fn sub(self, rhs: isize) -> Self::Output {
        Point {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Sub<usize> for Point {
    type Output = Point;
    
    fn sub(self, rhs: usize) -> Self::Output {
        Point {
            x: self.x - rhs as isize,
            y: self.y - rhs as isize,
        }
    }
}

impl Add for &Point {
    type Output = Point;
    
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<isize> for Point {
    type Output = Point;
    
    fn add(self, rhs: isize) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Add<usize> for Point {
    type Output = Point;
    
    fn add(self, rhs: usize) -> Self::Output {
        Point {
            x: self.x + rhs as isize,
            y: self.y + rhs as isize,
        }
    }
}