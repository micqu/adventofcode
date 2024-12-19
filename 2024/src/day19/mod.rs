use std::collections::VecDeque;

use itertools::Itertools;
use rayon::result;

use crate::utils::{
    solution::{IntoSolution, Solution},
    ToNumbers,
};

pub const TITLE: &str = "Linen Layout";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (patterns, designs) = parse();

    let mut s = 0;
    for d in designs {
        let mut cache = vec![None; d.len()];
        if solve(0, &d, None, &patterns, &mut cache) {
            s += 1;
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let (patterns, designs) = parse();

    let mut s = 0;
    for d in designs {
        let mut cache = vec![None; d.len()];
        s += solve2(0, &d, None, &patterns, &mut cache);
    }

    s.solution()
}

fn solve(
    i: usize,
    design: &Vec<u8>,
    child_patterns: Option<&Vec<Option<Node>>>,
    patterns: &Vec<Option<Node>>,
    cache: &mut Vec<Option<bool>>,
) -> bool {
    if i >= design.len() {
        return false;
    }

    if child_patterns.is_none() {
        if let Some(c) = cache[i] {
            return c;
        }
    }

    let mut result: bool = false;
    if let Some(n) = &child_patterns.unwrap_or(patterns)[map_id(design[i])] {
        if n.end {
            if i == design.len() - 1 {
                result = true;
            } else {
                result = solve(i + 1, design, None, patterns, cache)
                    || solve(i + 1, design, Some(&n.next), patterns, cache);
            }
        } else {
            result = solve(i + 1, design, Some(&n.next), patterns, cache);
        }
    }

    if child_patterns.is_none() {
        cache[i] = Some(result);
    }

    result
}

fn solve2(
    i: usize,
    design: &Vec<u8>,
    child_patterns: Option<&Vec<Option<Node>>>,
    patterns: &Vec<Option<Node>>,
    cache: &mut Vec<Option<usize>>,
) -> usize {
    if i >= design.len() {
        return 0;
    }

    if child_patterns.is_none() {
        if let Some(c) = cache[i] {
            return c;
        }
    }

    let mut result = 0;
    if let Some(n) = &child_patterns.unwrap_or(patterns)[map_id(design[i])] {
        if n.end {
            if i == design.len() - 1 {
                result = 1;
            } else {
                result = solve2(i + 1, design, None, patterns, cache)
                    + solve2(i + 1, design, Some(&n.next), patterns, cache);
            }
        } else {
            result = solve2(i + 1, design, Some(&n.next), patterns, cache);
        }
    }

    if child_patterns.is_none() {
        cache[i] = Some(result);
    }

    result
}

fn parse() -> (Vec<Option<Node>>, Vec<Vec<u8>>) {
    let mut patterns = vec![None; 5];
    let mut lines = INPUT.lines();

    if let Some(line) = lines.next() {
        for pattern in line.split(", ") {
            let mut bytes = pattern.bytes().collect();
            set(&mut bytes, &mut patterns);
        }
    }

    lines.next();

    let mut designs = Vec::new();
    while let Some(line) = lines.next() {
        designs.push(line.bytes().collect_vec());
    }

    (patterns, designs)
}

fn set(input: &mut VecDeque<u8>, patterns: &mut Vec<Option<Node>>) {
    if let Some(u) = input.pop_front() {
        let id = map_id(u);
        if patterns[id].is_none() {
            patterns[id] = Some(Node {
                end: if input.len() == 0 { true } else { false },
                next: vec![None; 5],
            });
        }

        if let Some(n) = &mut patterns[id] {
            if input.len() == 0 {
                n.end = true;
            }
            set(input, &mut n.next);
        }
    }
}

fn map_id(byte: u8) -> usize {
    match byte {
        b'w' => 0,
        b'u' => 1,
        b'b' => 2,
        b'r' => 3,
        b'g' => 4,
        _ => panic!(),
    }
}

#[derive(Debug, Clone)]
struct Node {
    end: bool,
    next: Vec<Option<Node>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (285 as i32).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (636483903099279 as usize).solution());
    }
}
