use std::collections::HashSet;

use num::integer::Roots;

use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Gift Shop";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut ids = 0;
    let mut bytes = INPUT.bytes();

    while let Some(from) = bytes.next_number() {
        let to = bytes.next_number().unwrap();

        for len in length(from)..=length(to) {
            if len % 2 == 0 {
                let o = 10_usize.pow(len / 2 - 1);
                let k = o * 10 + 1;

                for j in o..o * 10 {
                    let n = k * j;

                    if n > to {
                        break;
                    }

                    if n >= from {
                        ids += n;
                    }
                }
            }
        }
    }

    ids.solution()
}

pub fn part2() -> Option<Solution> {
    let mut ids = HashSet::<usize>::new();
    let mut bytes = INPUT.bytes();

    while let Some(from) = bytes.next_number() {
        let to = bytes.next_number().unwrap();

        for len in length(from)..=length(to) {
            for i in 1..=len / 2 {
                if len % i == 0 {
                    let o = 10_usize.pow(i - 1);
                    let k = mirror(o, i, len / i) / o;

                    for j in o..o * 10 {
                        let n = k * j;

                        if n > to {
                            break;
                        }

                        if n >= from {
                            ids.insert(n);
                        }
                    }
                }
            }
        }
    }

    ids.iter().map(|x| *x).sum::<usize>().solution()
}

fn length(a: usize) -> u32 {
    a.checked_ilog10().unwrap_or(0) + 1
}

fn mirror(a: usize, len: u32, n: u32) -> usize {
    let mut r = a;
    let m = 10_usize.pow(len);
    for _ in 1..n {
        r = r * m + a;
    }
    r
}

#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (5398419778 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (15704845910 as usize).solution());
    }
}
