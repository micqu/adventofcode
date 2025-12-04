use itertools::Itertools;

use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Lobby";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    INPUT
        .lines()
        .filter_map(|line| solve(line.as_bytes(), 0, 2))
        .sum::<usize>()
        .solution()
}

pub fn part2() -> Option<Solution> {
    INPUT
        .lines()
        .filter_map(|line| solve(line.as_bytes(), 0, 12))
        .sum::<usize>()
        .solution()
}

fn solve(bytes: &[u8], pos: usize, rem: usize) -> Option<usize> {
    if rem == 0 {
        return None;
    }

    let mut max = bytes[pos];
    let mut idx = pos;

    for i in pos + 1..bytes.len() - rem + 1 {
        let a = bytes[i];
        if a > max {
            max = a;
            idx = i;
        }
    }

    if let Some(s) = solve(bytes, idx + 1, rem - 1) {
        Some((max - b'0') as usize * 10_usize.pow(rem as u32 - 1) + s)
    } else {
        Some((max - b'0') as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (17412 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (172681562473501 as usize).solution());
    }
}
