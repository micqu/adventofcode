use std::{collections::HashMap, iter::from_fn};

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
        let mut next = HashMap::<usize, usize>::new();

        for (&k, v) in numbers.iter() {
            if *v == 0 {
                continue;
            }

            if k == 0 {
                *next.entry(1).or_insert(0) += *v;
                continue;
            }
            
            let l = length(k);
            if l % 2 == 0 {
                let d = 10usize.pow(l / 2);
                let a = k / d;
                let b = k % d;
                *next.entry(a).or_insert(0) += *v;
                *next.entry(b).or_insert(0) += *v;
                continue;
            }

            *next.entry(k * 2024).or_insert(0) += *v;
        }

        *numbers = next;
    }
}

fn length(mut a: usize) -> u32 {
    let mut i = 1;
    while a >= 10 {
        a /= 10;
        i += 1;
    }
    i
}

// fn parse() -> Vec<usize> {
//     let mut numbers = INPUT.bytes();
//     from_fn(|| numbers.next_number()).collect()
// }

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
    //238317474993392
    //10774622291287489000
}