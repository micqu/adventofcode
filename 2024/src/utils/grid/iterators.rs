pub const ADJ_FOUR: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];
pub const ADJ_DIAGONAL: [(isize, isize); 4] = [(1, -1), (-1, -1), (-1, 1), (1, 1)];
pub const ADJ_EIGHT: [(isize, isize); 8] = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub struct EightConnected {
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
    pub current: usize,
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

pub struct FourConnected {
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
    pub current: usize,
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
    pub x: isize,
    pub y: isize,
    pub height: usize,
    pub width: usize,
    pub current: usize,
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

            return Some((nx, ny, self.current - 1));
        }
    }
}

pub struct Diagonals {
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
    pub current: usize,
}

impl Iterator for Diagonals {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > 3 {
                return None;
            }

            let nx = self.x as isize + ADJ_DIAGONAL[self.current].0;
            let ny = self.y as isize + ADJ_DIAGONAL[self.current].1;

            self.current += 1;

            if nx < 0 || nx >= self.width as isize || ny < 0 || ny >= self.height as isize {
                continue;
            }

            return Some((nx as usize, ny as usize, self.current - 1));
        }
    }
}

pub struct Positions {
    pub x: usize,
    pub y: usize,
    pub height: usize,
    pub width: usize,
    pub index: usize,
}

impl Iterator for Positions {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let p = (self.index % self.width, self.index / self.width);

        if self.index < self.height * self.width {
            self.index += 1;
        } else {
            return None;
        }

        return Some(p);
    }
}