use std::iter::from_fn;

use crate::utils::{
    solution::{IntoSolution, Solution}, NextNumbers, Parsable
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
        .filter_map(|x| solve2(x.test, 1, &x.numbers).then_some(x.test))
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

fn solve2(t: usize, i: usize, e: &Vec<usize>) -> bool {
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

    if let Some(r) = split(t, d) {
        if solve2(r, i + 1, e) {
            return true;
        }
    }
    
    if t % d == 0 {
        solve2(t / d, i + 1, e) || solve2(t - d, i + 1, e)
    } else {
        solve2(t - d, i + 1, e)
    }
}

fn split(mut lhs: usize, mut rhs: usize) -> Option<usize> {
    while rhs > 0 {
        if lhs % 10 != rhs % 10 {
            return None;
        }
        lhs /= 10;
        rhs /= 10;
    }
    Some(lhs)
}

// fn solve2_forward(t: usize, i: usize, d: usize, e: &Vec<usize>) -> bool {
//     if i == e.len() {
//         return d == t;
//     }

//     if d > t {
//         return false;
//     }

//     solve2_forward(t, i + 1, concat(d, e[i]), e)
//         || solve2_forward(t, i + 1, d * e[i], e)
//         || solve2_forward(t, i + 1, d + e[i], e)
// }

// fn concat(a: usize, b: usize) -> usize {
//     a * 10usize.pow(length(b)) + b
// }

// fn length(mut a: usize) -> u32 {
//     let mut i = 1;
//     while a >= 10 {
//         a /= 10;
//         i += 1;
//     }
//     i
// }

fn parse() -> Vec<Equation> {
    INPUT
        .lines()
        .map(|line| {
            let mut numbers = line.bytes();
            Equation {
                test: numbers.next_number().unwrap(),
                numbers: numbers.next_numbers()
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
        assert_eq!(super::part1(), (663613490587 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (110365987435001 as usize).solution());
    }
}