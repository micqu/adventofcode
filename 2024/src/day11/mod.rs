use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{solution::{IntoSolution, Solution}, Parsable};

pub const TITLE: &str = "Plutonian Pebbles";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut numbers = parse();
    solve(25, &mut numbers);
    numbers.iter().map(|(_, v)| v).sum::<usize>().solution()
}

pub fn part2() -> Option<Solution> {
    let mut numbers = parse();
    solve(75, &mut numbers);
    numbers.iter().map(|(_, v)| v).sum::<usize>().solution()
}

fn solve(n: usize, numbers: &mut HashMap::<usize, usize>) {
    for _ in 0..n {
        for (k, v) in numbers.drain().collect_vec() {
            if k == 0 {
                *numbers.entry(1).or_insert(0) += v;
                continue;
            }
            
            let l = k.ilog10() + 1;
            if l % 2 == 0 {
                let d = 10usize.pow(l / 2);
                *numbers.entry(k / d).or_insert(0) += v;
                *numbers.entry(k % d).or_insert(0) += v;
                continue;
            }

            *numbers.entry(k * 2024).or_insert(0) += v;
        }
    }
}

fn parse() -> HashMap::<usize, usize> {
    let mut map = HashMap::<usize, usize>::new();
    let mut numbers = INPUT.bytes();
    while let Some(n) = numbers.next_number() {
        *map.entry(n).or_insert(0) += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (200446 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (238317474993392 as usize).solution());
    }
}