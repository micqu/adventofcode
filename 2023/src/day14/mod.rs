use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
};

use itertools::Itertools;

use crate::utils::solution::{IntoSolution, Solution};

pub const TITLE: &str = "Parabolic Reflector Dish";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    parse().up().total_load().solution()
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

struct Map {
    content: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Map {
    fn total_load(&self) -> usize {
        let mut s = 0;
        for i in 0..self.height {
            s += bytecount::count(&self.content[i], b'O') * (self.height - i);
        }
        s
    }

    fn cycle(&mut self) -> &Self {
        self.up().left().down().right()
    }

    fn up(&mut self) -> &mut Self {
        for x in 0..self.width {
            let mut c = 0;
            for y in (0..self.height).rev() {
                match self.content[y][x] {
                    b'#' => {
                        for v in (y + 1)..(y + 1 + c) {
                            self.content[v][x] = b'O';
                        }
                        c = 0;
                    }
                    b'O' => {
                        self.content[y][x] = b'.';
                        c += 1;
                    }
                    _ => {}
                }
            }

            for v in 0..c {
                self.content[v][x] = b'O';
            }
        }
        self
    }

    fn left(&mut self) -> &mut Self {
        for y in 0..self.height {
            let mut c = 0;
            for x in (0..self.width).rev() {
                match self.content[y][x] {
                    b'#' => {
                        for v in (x + 1)..(x + 1 + c) {
                            self.content[y][v] = b'O';
                        }
                        c = 0;
                    }
                    b'O' => {
                        self.content[y][x] = b'.';
                        c += 1;
                    }
                    _ => {}
                }
            }

            for v in 0..c {
                self.content[y][v] = b'O';
            }
        }
        self
    }

    fn down(&mut self) -> &mut Self {
        for x in 0..self.width {
            let mut c = 0;
            for y in 0..self.height {
                match self.content[y][x] {
                    b'#' => {
                        for v in (y - c)..y {
                            self.content[v][x] = b'O';
                        }
                        c = 0;
                    }
                    b'O' => {
                        self.content[y][x] = b'.';
                        c += 1;
                    }
                    _ => {}
                }
            }

            for v in self.height - c..self.height {
                self.content[v][x] = b'O';
            }
        }
        self
    }

    fn right(&mut self) -> &mut Self {
        for y in 0..self.height {
            let mut c = 0;
            for x in 0..self.width {
                match self.content[y][x] {
                    b'#' => {
                        for v in (x - c)..x {
                            self.content[y][v] = b'O';
                        }
                        c = 0;
                    }
                    b'O' => {
                        self.content[y][x] = b'.';
                        c += 1;
                    }
                    _ => {}
                }
            }

            for v in self.width - c..self.width {
                self.content[y][v] = b'O';
            }
        }
        self
    }

    fn print(&self) -> &Self {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.content[i][j] == b'O' {
                    print!("O")
                } else if self.content[i][j] == b'#' {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
        self
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        for line in self.content.iter() {
            hasher.write(&line);
        }
        hasher.finish()
    }
}

fn parse() -> Map {
    let map = INPUT
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();
    
    let h = map.len();
    let w = map[0].len();

    Map { content: map, height: h, width: w }
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
