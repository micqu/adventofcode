use std::collections::HashMap;

use crate::utils::{solution::{IntoSolution, Solution}, vec2d::Vec2d};

pub const TITLE: &str = "Parabolic Reflector Dish";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    parse().tilt_up().solution()
}

pub fn part2() -> Option<Solution> {
    let mut map = parse();

    const N: usize = 1_000_000_000;
    let mut seen = HashMap::<u64, usize>::new();
    let mut cycle_end = 0;
    let mut hash = 0;
    for i in 0..N {
        map.tilt_up();
        map.tilt_left();
        let d = map.tilt_down();
        let r = map.tilt_right();

        hash = ((r as u64) << 32) | d as u64;

        if seen.contains_key(&hash) {
            cycle_end = i + 1;
            break;
        }

        seen.insert(hash, i + 1);
    }

    let cycle_begin = seen.get(&hash).unwrap();
    let cycle_len = cycle_end - cycle_begin;
    let rem = (N - cycle_end) % cycle_len;
    let end = seen.iter().find(|x| *x.1 == cycle_begin + rem).unwrap();
    (end.0 >> 32).solution()
}

trait ParabolicDish {
    fn tilt_up(&mut self) -> u32;
    fn tilt_down(&mut self) -> u32;
    fn tilt_left(&mut self) -> u32;
    fn tilt_right(&mut self) -> u32;
    fn print(&self);
}

impl ParabolicDish for Vec2d<u8> {
    fn tilt_up(&mut self) -> u32 {
        let mut s = 0;
        for x in 0..self.width {
            let mut c = 0;
            let mut p = 0;
            for y in 0..self.height {
                match self.index(x, y) {
                    b'O' => {
                        *self.index_mut(x, y) = b'.';
                        *self.index_mut(x, p + c) = b'O';
                        s += (self.height - (p + c)) as u32;
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
        s
    }

    fn tilt_left(&mut self) -> u32 {
        let mut s = 0;
        for y in 0..self.height {
            let mut c = 0;
            let mut p = 0;
            for x in 0..self.width {
                match self.index(x, y) {
                    b'O' => {
                        *self.index_mut(x, y) = b'.';
                        *self.index_mut(p + c, y) = b'O';
                        s += (self.height - y) as u32;
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
        s
    }

    fn tilt_down(&mut self) -> u32 {
        let mut s = 0;
        for x in 0..self.width {
            let mut c = 0;
            let mut p = self.height - 1;
            for y in (0..self.height).rev() {
                match self.index(x, y) {
                    b'O' => {
                        *self.index_mut(x, y) = b'.';
                        *self.index_mut(x, p - c) = b'O';
                        s += (self.height - (p - c)) as u32;
                        c += 1;
                    }
                    b'#' => {
                        p = y.saturating_sub(1);
                        c = 0;
                    }
                    _ => {}
                }
            }
        }
        s
    }

    fn tilt_right(&mut self) -> u32 {
        let mut s = 0;
        for y in 0..self.height {
            let mut c = 0;
            let mut p = self.width - 1;
            for x in (0..self.width).rev() {
                match self.index(x, y) {
                    b'O' => {
                        *self.index_mut(x, y) = b'.';
                        *self.index_mut(p - c, y) = b'O';
                        s += (self.height - y) as u32;
                        c += 1;
                    }
                    b'#' => {
                        p = x.saturating_sub(1);
                        c = 0;
                    }
                    _ => {}
                }
            }
        }
        s
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
            Solution::U32(a) => assert_eq!(a, 110565),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::U64(a) => assert_eq!(a, 89845),
            _ => panic!(),
        }
    }
}
