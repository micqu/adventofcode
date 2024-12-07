use std::{collections::HashMap, iter::from_fn};

use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Bridge Repair";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    parse()
        .iter()
        .filter_map(|x| solve(x.test, 1, &x.numbers).then_some(x.test))
        .sum::<usize>()
        .solution()
}

pub fn part2() -> Option<Solution> {
    parse()
        .iter()
        .filter_map(|x| solve2(x.test, 1, x.numbers[0], &x.numbers).then_some(x.test))
        .sum::<usize>()
        .solution()
}

fn solve(t: usize, i: usize, e: &Vec<usize>) -> bool {
    if i > e.len() {
        return false;
    }

    let d = e[e.len() - i];
    if d > t {
        return false;
    }

    if d == t {
        return true;
    }

    if t % d == 0 {
        solve(t / d, i + 1, e) || solve(t - d, i + 1, e)
    } else {
        solve(t - d, i + 1, e)
    }
}

fn solve2(t: usize, i: usize, d: usize, e: &Vec<usize>) -> bool {
    if i == e.len() {
        return d == t;
    }

    if d > t {
        return false;
    }

    solve2(t, i + 1, concat(d, e[i]), e)
        || solve2(t, i + 1, d * e[i], e)
        || solve2(t, i + 1, d + e[i], e)
}

fn concat(a: usize, b: usize) -> usize {
    a * 10usize.pow(length(b)) + b
}

fn length(mut a: usize) -> u32 {
    let mut i = 1;
    while a >= 10 {
        a /= 10;
        i += 1;
    }
    i
}

fn parse() -> Vec<Equation> {
    INPUT
        .lines()
        .map(|line| {
            let mut numbers = line.bytes();
            Equation {
                test: numbers.next_number().unwrap(),
                numbers: from_fn(|| numbers.next_number()).collect(),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Equation {
    test: usize,
    numbers: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 663613490587),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 110365987435001),
            _ => panic!(),
        }
    }
}
