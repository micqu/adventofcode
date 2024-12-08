use core::fmt;

#[derive(Debug, Clone)]
pub struct Vec2d<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Vec2d<T> {
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
        Self { data: vec, width, height }
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

    pub fn contains(&self, p: &(isize, isize)) -> Option<(usize, usize)> {
        if p.0 < 0 || p.0 >= self.width as isize || p.1 < 0 || p.1 >= self.height as isize {
            return None;
        }
    
        Some((p.0 as usize, p.1 as usize))
    }
}

impl<T> std::ops::Index<(usize, usize)> for Vec2d<T> {
    fn index(&self, index: (usize, usize)) -> &T {
        self.index(index.0, index.1)
    }

    type Output = T;
}

impl<T> std::ops::IndexMut<(usize, usize)> for Vec2d<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        self.index_mut(index.0, index.1)
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Vec2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.height {
            if i != 0 {
                str.push_str(", ");
            }
            str.push_str(&format!("{:?}", &self.row(i)));
            str.push('\n');
        }
        write!(f, "{}", str)
    }
}

pub const ADJ_EIGHT: [(isize, isize); 8] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, 0),
    (1, 1),
    (1, -1),
];

pub struct EightConnected {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    current: usize,
}

impl Iterator for EightConnected {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 7 {
                return None;
            }

            let nx = self.x as isize + ADJ_EIGHT[self.current].0;
            let ny = self.y as isize + ADJ_EIGHT[self.current].1;

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some((nx as usize, ny as usize));
        }
    }
}

pub const ADJ_FOUR: [(isize, isize); 4] = [ (1, 0), (0, -1), (-1, 0), (0, 1) ];

pub struct FourConnected {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    current: usize,
}

impl Iterator for FourConnected {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 3 {
                return None;
            }

            let nx = self.x as isize + ADJ_FOUR[self.current].0;
            let ny = self.y as isize + ADJ_FOUR[self.current].1;

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some((nx as usize, ny as usize, self.current - 1));
        }
    }
}

pub struct FourConnectedUnbound {
    x: isize,
    y: isize,
    height: usize,
    width: usize,
    current: usize,
}

impl Iterator for FourConnectedUnbound {
    type Item = (isize, isize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 3 {
                return None;
            }

            let nx = self.x + ADJ_FOUR[self.current].0;
            let ny = self.y + ADJ_FOUR[self.current].1;

            self.current += 1;

            return Some((nx, ny, self.current));
        }
    }
}

pub const DIAGONAL: [(isize, isize); 4] = [(-1, 1), (1, -1), (1, 1), (-1, -1)];

pub struct Diagonals {
    x: usize,
    y: usize,
    height: usize,
    width: usize,
    current: usize,
}

impl Iterator for Diagonals {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 3 {
                return None;
            }

            let nx = self.x as isize + DIAGONAL[self.current].0;
            let ny = self.y as isize + DIAGONAL[self.current].1;

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some((nx as usize, ny as usize));
        }
    }
}
