use std::{
    cmp::Reverse,
    ops::{Range, RangeInclusive},
};

use itertools::Itertools;

use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Cafeteria";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (ranges, ids) = parse();

    let mut s: usize = 0;
    for id in ids {
        for r in &ranges {
            if id >= *r.start() && id <= *r.end() {
                s += 1;
                break;
            }
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let (mut ranges, _) = parse();

    let mut merged = Vec::new();
    let mut c = ranges.pop().unwrap();

    while let Some(range) = ranges.pop() {
        if range.contains(&c.start()) {
            c = *range.start()..=*c.end().max(range.end());
        } else {
            merged.push(c);
            c = range;
        }
    }

    merged.push(c);

    merged
        .iter()
        .map(|x| x.end() - x.start() + 1)
        .sum::<usize>()
        .solution()
}

fn parse() -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let mut parse_ranges = true;
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    for line in INPUT.lines() {
        let mut bytes = line.bytes();

        if parse_ranges {
            if line.is_empty() {
                parse_ranges = false;
            } else {
                let from: usize = bytes.next_number().unwrap();
                let to: usize = bytes.next_number().unwrap();
                ranges.push(from..=to);
            }
        } else {
            while let Some(n) = bytes.next_number() {
                ids.push(n);
            }
        }
    }

    ranges.sort_unstable_by_key(|a| *a.start());

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (513 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (339668510830757 as usize).solution());
    }
}
