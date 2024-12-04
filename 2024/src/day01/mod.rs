use std::collections::HashMap;

use crate::utils::{solution::{self, IntoSolution, Solution}, Parsable, ToNumbers, Transposable};

pub const TITLE: &str = "Historian Hysteria";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut a = Vec::<u32>::with_capacity(1000);
    let mut b = Vec::<u32>::with_capacity(1000);
    INPUT.lines().for_each(|line| {
        let mut bytes = line.bytes();
        a.push(bytes.next_number().unwrap());
        b.push(bytes.next_number().unwrap());
    });
    a.sort_unstable();
    b.sort_unstable();

    a.iter().zip(b).map(|(x, y)| x.abs_diff(y)).sum::<u32>().solution()
}

pub fn part2() -> Option<Solution> {
    let mut a = Vec::<u32>::with_capacity(1000);
    let mut b = HashMap::<u32, u32>::with_capacity(1000);
    INPUT.lines().for_each(|line| {
        let mut bytes = line.bytes();
        let x = bytes.next_number().unwrap();
        let y = bytes.next_number().unwrap();
        a.push(x);
        b.entry(x).or_insert(0);
        *b.entry(y).or_insert(0) += 1;
    });

    a.iter().map(|x| x * b[x]).sum::<u32>().solution()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 2196996),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 23655822),
            _ => panic!(),
        }
    }
}