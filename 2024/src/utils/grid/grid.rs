use core::fmt;

use crate::utils::points::point2d::Point2d;

use super::iterators::{Diagonals, EightConnected, FourConnected, FourConnectedUnbound, Positions};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn from(value: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Self {
            height,
            width,
            data: vec![value; width * height],
        }
    }

    pub fn from_vec_width(vec: Vec<T>, width: usize) -> Self {
        Self {
            height: vec.len() / width,
            width,
            data: vec,
        }
    }

    pub fn from_vec_height(vec: Vec<T>, height: usize) -> Self {
        Self {
            height,
            width: vec.len() / height,
            data: vec,
        }
    }

    pub fn new(vec: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            data: vec,
            width,
            height,
        }
    }

    pub fn row(&self, y: usize) -> &[T] {
        &self.data[self.width * y..(self.width * y + self.width)]
    }

    pub fn row_mut(&mut self, y: usize) -> &mut [T] {
        &mut self.data[self.width * y..(self.width * y + self.width)]
    }

    pub fn index(&self, x: usize, y: usize) -> &T {
        &self.data[self.width * y + x]
    }

    pub fn index_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[self.width * y + x]
    }

    pub fn up(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if y <= 0 {
            return None;
        }

        Some((x, y - 1))
    }

    pub fn down(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if y >= self.height - 1 {
            return None;
        }

        Some((x, y + 1))
    }

    pub fn right(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x >= self.width - 1 {
            return None;
        }

        Some((x + 1, y))
    }

    pub fn left(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x <= 0 {
            return None;
        }

        Some((x - 1, y))
    }

    pub fn contains(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            return None;
        }

        Some((x as usize, y as usize))
    }

    pub fn contains_point(&self, p: &(isize, isize)) -> Option<(usize, usize)> {
        if p.0 < 0 || p.0 >= self.width as isize || p.1 < 0 || p.1 >= self.height as isize {
            return None;
        }

        Some((p.0 as usize, p.1 as usize))
    }

    pub fn contains_point2d(&self, p: &Point2d) -> bool {
        !(p.x < 0 || p.x >= self.width as isize || p.y < 0 || p.y >= self.height as isize)
    }

    pub fn eight_connected(&self, x: usize, y: usize) -> EightConnected {
        EightConnected {
            x,
            y,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn eight_connected_point(&self, point: (usize, usize)) -> EightConnected {
        EightConnected {
            x: point.0,
            y: point.1,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn four_connected(&self, x: usize, y: usize) -> FourConnected {
        FourConnected {
            x,
            y,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn four_connected_point(&self, point: (usize, usize)) -> FourConnected {
        FourConnected {
            x: point.0,
            y: point.1,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn four_connected_point2d(&self, point: &Point2d) -> FourConnected {
        FourConnected {
            x: point.x as usize,
            y: point.y as usize,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn four_connected_unbound(&self, x: isize, y: isize) -> FourConnectedUnbound {
        FourConnectedUnbound {
            x: x,
            y: y,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn diagonals(&self, x: usize, y: usize) -> Diagonals {
        Diagonals {
            x,
            y,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn diagonals_point(&self, point: (usize, usize)) -> Diagonals {
        Diagonals {
            x: point.0,
            y: point.1,
            height: self.height,
            width: self.width,
            current: 0,
        }
    }

    pub fn positions(&self) -> Positions {
        Positions {
            x: 0,
            y: 0,
            height: self.height,
            width: self.width,
            index: 0,
        }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    fn index(&self, index: (usize, usize)) -> &T {
        self.index(index.0, index.1)
    }

    type Output = T;
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        self.index_mut(index.0, index.1)
    }
}

impl<T> std::ops::Index<(isize, isize)> for Grid<T> {
    fn index(&self, index: (isize, isize)) -> &T {
        self.index(index.0 as usize, index.1 as usize)
    }

    type Output = T;
}

impl<T> std::ops::IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut T {
        self.index_mut(index.0 as usize, index.1 as usize)
    }
}

impl<T> std::ops::Index<Point2d> for Grid<T> {
    fn index(&self, index: Point2d) -> &T {
        self.index(index.x as usize, index.y as usize)
    }

    type Output = T;
}

impl<T> std::ops::IndexMut<Point2d> for Grid<T> {
    fn index_mut(&mut self, index: Point2d) -> &mut T {
        self.index_mut(index.x as usize, index.y as usize)
    }
}

impl<T> std::ops::Index<&Point2d> for Grid<T> {
    fn index(&self, index: &Point2d) -> &T {
        self.index(index.x as usize, index.y as usize)
    }

    type Output = T;
}

impl<T> std::ops::IndexMut<&Point2d> for Grid<T> {
    fn index_mut(&mut self, index: &Point2d) -> &mut T {
        self.index_mut(index.x as usize, index.y as usize)
    }
}

// impl<T: std::fmt::Debug> std::fmt::Display for Vec2d<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut str = String::new();
//         for i in 0..self.height {
//             if i != 0 {
//                 str.push_str(", ");
//             }
//             str.push_str(&format!("{:?}", &self.row(i)));
//             str.push('\n');
//         }
//         write!(f, "{}", str)
//     }
// }

impl std::fmt::Display for Grid<bool> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.height {
            str.push_str(" ");
            let row = self.row(i);
            for j in 0..self.width {
                if row[j] {
                    str.push('#');
                } else {
                    str.push_str(" ");
                }
            }
            str.push('\n');
        }
        write!(f, "{}", str)
    }
}


impl std::fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.height {
            str.push_str(" ");
            let row = self.row(i);
            for j in 0..self.width {
                let byte = row[j];
                str.push(byte as char);
            }
            str.push('\n');
        }
        write!(f, "{}", str)
    }
}