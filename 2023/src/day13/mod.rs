use crate::utils::solution::{IntoSolution, Solution};

pub const TITLE: &str = "Point of Incidence";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    parse()
        .iter()
        .map(|map| {
            for v in 1..map.width() {
                if check(&map.cols[0..v], &map.cols[v..]) {
                    return v;
                }
            }

            for v in 1..map.height() {
                if check(&map.rows[0..v], &map.rows[v..]) {
                    return v * 100;
                }
            }

            0
        })
        .sum::<usize>()
        .solution()
}

pub fn part2() -> Option<Solution> {
    parse()
        .iter()
        .map(|map| {
            for v in 1..map.width() {
                if check_smudge(&map.cols[0..v], &map.cols[v..], 1) {
                    return v;
                }
            }

            for v in 1..map.height() {
                if check_smudge(&map.rows[0..v], &map.rows[v..], 1) {
                    return v * 100;
                }
            }

            0
        })
        .sum::<usize>()
        .solution()
}

fn parse() -> Vec<Bitmap> {
    let mut maps = vec![Bitmap::new()];
    for line in INPUT.lines() {
        if line.is_empty() {
            maps.push(Bitmap::new());
            continue;
        }

        let map = maps.last_mut().unwrap();
        let mut row: u32 = 0;
        for (x, byte) in line.bytes().enumerate() {
            if map.cols.len() < x + 1 {
                map.cols.push(0);
            }

            match byte {
                b'#' => {
                    row = (row << 1) | 1;
                    map.cols[x] = (map.cols[x] << 1) | 1;
                }
                b'.' => {
                    row <<= 1;
                    map.cols[x] <<= 1;
                }
                _ => panic!(),
            }
        }

        map.rows.push(row);
    }

    maps
}

fn check(first: &[u32], last: &[u32]) -> bool {
    let mut f = first.iter().rev();
    let mut l = last.iter();
    while let Some(a) = f.next() {
        if let Some(b) = l.next() {
            if a != b {
                return false;
            }
        } else {
            break;
        }
    }

    true
}

fn check_smudge(first: &[u32], last: &[u32], smudges: u32) -> bool {
    let mut f = first.iter().rev();
    let mut l = last.iter();
    let mut used = 0;
    while let Some(a) = f.next() {
        if let Some(b) = l.next() {
            used += (a ^ b).count_ones();
            if used > smudges {
                return false;
            }
        } else {
            break;
        }
    }

    used == smudges
}

#[derive(Debug)]
struct Bitmap {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Bitmap {
    fn new() -> Self {
        Self {
            rows: Vec::with_capacity(20),
            cols: Vec::with_capacity(20),
        }
    }

    fn width(&self) -> usize {
        self.cols.len()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn at(&self, x: usize, y: usize) -> bool {
        self.rows[y] & (1 << (self.width() - x - 1)) != 0
    }

    fn print(&self) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                if self.at(j, i) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 29130),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 33438),
            _ => panic!(),
        }
    }
}
