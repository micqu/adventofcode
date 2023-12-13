use std::collections::HashMap;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::utils::{
    solution::{IntoSolution, Solution},
    ToNumbers,
};

pub const TITLE: &str = "Hot Springs";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    INPUT
        .lines()
        .par_bridge()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();
            let counts = counts.to_numbers();
            let mut cache = HashMap::<(u32, u32), u64>::new();
            solve(springs.as_bytes(), &counts, 0, 0, &mut cache)
        })
        .sum::<u64>()
        .solution()
}

pub fn part2() -> Option<Solution> {
    INPUT
        .lines()
        .par_bridge()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();
            let mut springs = format!("{}?", springs).repeat(5);
            springs.remove(springs.len() - 1);
            
            let counts = counts.to_numbers().repeat(5);
            let mut cache = HashMap::<(u32, u32), u64>::new();
            solve(springs.as_bytes(), &counts, 0, 0, &mut cache)
        })
        .sum::<u64>()
        .solution()
}

fn mem_wrap(
    springs: &[u8],
    counts: &Vec<usize>,
    spring_idx: usize,
    count_idx: usize,
    mem: &mut HashMap<(u32, u32), u64>,
) -> u64 {
    let h = (spring_idx as u32, count_idx as u32);
    if let Some(v) = mem.get(&h) {
        return *v;
    } else {
        let r = solve(springs, counts, spring_idx, count_idx, mem);
        mem.insert(h, r);
        return r;
    }
}

fn solve(
    springs: &[u8],
    counts: &Vec<usize>,
    spring_idx: usize,
    count_idx: usize,
    mem: &mut HashMap<(u32, u32), u64>,
) -> u64 {
    if spring_idx >= springs.len() {
        return (count_idx >= counts.len()) as u64;
    }

    if count_idx >= counts.len() {
        return !springs[spring_idx..].contains(&b'#') as u64;
    }

    if count_idx >= counts.len() {
        return 0;
    }

    let mut s: u64 = 0;
    let block = counts[count_idx];
    for (start, window) in springs[spring_idx..].windows(block).enumerate() {
        if start > 0 && springs[spring_idx + start - 1] == b'#' {
            return s;
        }

        let after = spring_idx + start + block;

        if after < springs.len() && springs[after] == b'#' {
            continue;
        }

        if window.iter().contains(&b'.') {
            continue;
        }

        let mut next_idx = after + 1;
        while next_idx < springs.len() && springs[next_idx] == b'.' {
            next_idx += 1;
        }

        s += mem_wrap(springs, &counts, next_idx, count_idx + 1, mem);
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::U64(a) => assert_eq!(a, 8075),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::U64(a) => assert_eq!(a, 4232520187524),
            _ => panic!(),
        }
    }
}
