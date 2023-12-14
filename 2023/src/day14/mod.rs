use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
};

use crate::utils::{solution::{IntoSolution, Solution}, vec2d::Vec2d};

pub const TITLE: &str = "Parabolic Reflector Dish";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    parse().tilt_up().total_load().solution()
}

pub fn part2() -> Option<Solution> {
    let mut map = parse();

    const N: usize = 1_000_000_000;
    let mut seen = HashMap::<u64, usize>::new();
    let mut cycle_idx = 0;
    let mut hash: u64 = 0;
    for i in 0..N {
        hash = map.hash();

        if seen.contains_key(&hash) {
            cycle_idx = i;
            break;
        }

        seen.insert(hash, i);
        map.cycle();
    }

    if cycle_idx > 0 {
        let cycle_len = cycle_idx - seen.get(&hash).unwrap();
        let rem = (N - cycle_idx) % cycle_len;
        for _ in 0..rem {
            map.cycle();
        }
    }
    
    map.total_load().solution()
}

trait ParabolicDish {
    fn total_load(&self) -> usize;
    fn cycle(&mut self) -> &mut Self;
    fn tilt_up(&mut self) -> &mut Self;
    fn tilt_down(&mut self) -> &mut Self;
    fn tilt_left(&mut self) -> &mut Self;
    fn tilt_right(&mut self) -> &mut Self;
    fn hash(&self) -> u64;
    fn print(&self);
}

impl ParabolicDish for Vec2d<u8> {
    fn total_load(&self) -> usize {
        let mut s = 0;
        for i in 0..self.height {
            s += bytecount::count(&self.row(i), b'O') * (self.height - i);
        }
        s
    }

    fn cycle(&mut self) -> &mut Self {
        self.tilt_up().tilt_left().tilt_down().tilt_right()
    }

    fn tilt_up(&mut self) -> &mut Self {
        for x in 0..self.width {
            let mut c = 0;
            let mut p = 0;
            for y in 0..self.height {
                match self.index(x, y) {
                    b'O' => {
                        *self.index_mut(x, y) = b'.';
                        *self.index_mut(x, p + c) = b'O';
                        c += 1;
                    }
                    b'#' => {
                        p = y + 1;
                        c = 0;
                    }
                    _ => {}
                }
            }
        }
        self
    }

    fn tilt_left(&mut self) -> &mut Self {
        for y in 0..self.height {
            let mut c = 0;
            let mut p = 0;
            for x in 0..self.width {
                match self.index(x, y) {
                    b'O' => {
                        *self.index_mut(x, y) = b'.';
                        *self.index_mut(p + c, y) = b'O';
                        c += 1;
                    }
                    b'#' => {
                        p = x + 1;
                        c = 0;
                    }
                    _ => {}
                }
            }
        }
        self
    }

    fn tilt_down(&mut self) -> &mut Self {
        for x in 0..self.width {
            let mut c = 0;
            let mut p = self.height - 1;
            for y in (0..self.height).rev() {
                match self.index(x, y) {
                    b'O' => {
                        *self.index_mut(x, y) = b'.';
                        *self.index_mut(x, p - c) = b'O';
                        c += 1;
                    }
                    b'#' => {
                        p = y - 1;
                        c = 0;
                    }
                    _ => {}
                }
            }
        }
        self
    }

    fn tilt_right(&mut self) -> &mut Self {
        for y in 0..self.height {
            let mut c = 0;
            let mut p = self.width - 1;
            for x in (0..self.width).rev() {
                match self.index(x, y) {
                    b'O' => {
                        *self.index_mut(x, y) = b'.';
                        *self.index_mut(p - c, y) = b'O';
                        c += 1;
                    }
                    b'#' => {
                        p = x - 1;
                        c = 0;
                    }
                    _ => {}
                }
            }
        }
        self
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if *self.index(x, y) == b'O' {
                    print!("O")
                } else if *self.index(x, y) == b'#' {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        hasher.write(&self.data);
        hasher.finish()
    }
}

fn parse() -> Vec2d<u8> {
    let mut bytes = Vec::<u8>::new();
    let mut bytes_iter = INPUT.bytes();
    while let Some(byte) = bytes_iter.next() {
        match byte {
            b'\n' => { },
            v => bytes.push(v),
        }
    }

    Vec2d::from_vec(bytes, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 110565),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 89845),
            _ => panic!(),
        }
    }
}
