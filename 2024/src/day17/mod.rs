use std::{collections::BinaryHeap, iter::from_fn};

use itertools::Itertools;

use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Chronospatial Computer";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (mut registers, ops) = parse();
    
    let mut output = Vec::new();
    let mut i: isize = 0;

    while i < (ops.len() - 1) as isize {
        let op = ops[i as usize];
        let literal = ops[i as usize + 1];
        let combo = match literal {
            0..4 => literal,
            4..7 => registers[literal as usize - 4],
            _ => panic!(),
        };

        match op {
            0 => registers[0] >>= combo,
            1 => registers[1] ^= literal,
            2 => registers[1] = combo & 0b111,
            3 => {
                if registers[0] != 0 {
                    i = literal - 2;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => output.push(combo & 0b111),
            6 => registers[1] = registers[0] >> combo,
            7 => registers[2] = registers[0] >> combo,
            _ => panic!(),
        }
        i += 2;
    }

    output.into_iter().join(",").solution()
}

pub fn part2() -> Option<Solution> {
    let (_, ops) = parse();
    solve(0, ops[3], ops[7], &ops, ops.len() - 1, 0).unwrap().solution()
}

fn solve(
    a: isize,
    x: isize,
    y: isize,
    output: &Vec<isize>,
    index: usize,
    end: usize,
) -> Option<isize> {
    let mut results = Vec::new();

    for b in 0..8 {
        let a = a << 3 | b;
        if b ^ x ^ y ^ (a >> (b ^ x)) & 0b111 == output[index] {
            if index == end {
                results.push(a);
            } else if let Some(result) = solve(a, x, y, output, index - 1, end) {
                results.push(result);
            }
        }
    }

    results.into_iter().min()
}

fn parse() -> ([isize; 3], Vec<isize>) {
    let mut bytes = INPUT.bytes();
    let registers = [
        bytes.next_number().unwrap(),
        bytes.next_number().unwrap(),
        bytes.next_number().unwrap(),
    ];

    let ops = std::iter::from_fn(|| bytes.next_number()).collect_vec();
    (registers, ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), ("1,5,0,3,7,3,0,3,1").solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (105981155568026 as isize).solution());
    }
}
