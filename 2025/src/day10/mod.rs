use crate::utils::{
    Parsable,
    solution::{IntoSolution, Solution},
};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use std::{collections::VecDeque, iter::from_fn};

pub const TITLE: &str = "Factory";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut q = VecDeque::<State>::new();
    let mut seen = HashSet::<u16>::default();
    let mut s = 0;

    for (state, buttons) in parse() {
        seen.clear();
        q.clear();
        q.push_back(State { n: 0, steps: 0 });

        while let Some(u) = q.pop_front() {
            if u.n == state {
                s += u.steps;
                break;
            }

            for &n in &buttons {
                let next = u.n ^ n;

                if seen.insert(next) {
                    q.push_back(State {
                        n: next,
                        steps: u.steps + 1,
                    });
                }
            }
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    parse2()
        .iter()
        .filter_map(|(buttons, joltages)| {
            let combinations = (0..=buttons.len())
                .flat_map(|k| buttons.iter().combinations(k))
                .map(|q| {
                    let sum = q.iter().map(|x| **x).sum::<u128>();
                    let presses = q.iter().len();
                    (sum, presses)
                })
                .collect_vec();

            solve2(*joltages, &combinations, &mut HashMap::default())
        })
        .sum::<usize>()
        .solution()
}

fn solve2(
    joltages: u128,
    combinations: &Vec<(u128, usize)>,
    cache: &mut HashMap<u128, Option<usize>>,
) -> Option<usize> {
    if joltages == 0 {
        return Some(0);
    }

    if let Some(c) = cache.get(&joltages) {
        return *c;
    }

    let s = combinations
        .iter()
        .filter_map(|(sum, presses)| {
            if let Some(v) = checked_sub(joltages, *sum)
                && is_even(v)
                && let Some(r) = solve2(v >> 1, combinations, cache)
            {
                Some(2 * r + presses)
            } else {
                None
            }
        })
        .min();

    cache.insert(joltages, s);

    s
}

fn checked_sub(a: u128, b: u128) -> Option<u128> {
    for i in 0..10 {
        let mask = 0b1111111111 << (i * 10);
        if (a & mask) < (b & mask) {
            return None;
        }
    }

    Some(a - b)
}

fn is_even(a: u128) -> bool {
    for i in 0..10 {
        if a & (1 << (i * 10)) != 0 {
            return false;
        }
    }

    true
}

fn parse() -> Vec<(u16, Vec<u16>)> {
    let mut machines = Vec::new();
    for line in INPUT.lines() {
        let mut bytes = line.bytes();
        let mut state = 0;
        let mut size = 0;

        while let Some(b) = bytes.next() {
            match b {
                b'#' | b'.' => {
                    state = state * 2 + if b == b'#' { 1 } else { 0 };
                    size += 1;
                }
                b'[' => {}
                b']' => {
                    machines.push((state, Vec::new()));
                    break;
                }
                _ => break,
            }
        }

        let mut button = 0;
        while let Some(c) = bytes.next() {
            match c {
                b'{' => break,
                b')' => {
                    machines.last_mut().unwrap().1.push(button);
                    button = 0;
                }
                c => {
                    if c.is_ascii_digit() {
                        button |= 1 << (size - c - 1);
                    }
                }
            }
        }
    }

    machines
}

fn parse2() -> Vec<(Vec<u128>, u128)> {
    let mut s = Vec::new();

    for line in INPUT.lines() {
        let mut bytes = line.bytes();
        let mut buttons = Vec::new();
        let mut button = 0;

        while let Some(c) = bytes.next() {
            match c {
                b'{' => break,
                b' ' => {
                    buttons.push(button);
                    button = 0;
                }
                c => {
                    if c.is_ascii_digit() {
                        button |= 1 << (c - b'0') * 10;
                    }
                }
            }
        }

        let lights = from_fn(|| Parsable::<u128>::next_number(&mut bytes))
            .enumerate()
            .fold(0, |acc, (i, c)| acc + (c << i * 10));

        s.push((buttons, lights));
    }

    s
}

#[derive(Debug)]
struct State {
    n: u16,
    steps: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (469 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (19293 as usize).solution());
    }
}
