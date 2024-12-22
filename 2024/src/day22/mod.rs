use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::{
    solution::{IntoSolution, Solution},
    ToNumbers,
};

pub const TITLE: &str = "Monkey Market";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let secrets: Vec<isize> = INPUT.numbers();
    let mut s = 0;
    for mut secret in secrets {
        for _ in 0..2000 {
            evolve(&mut secret);
        }
        s += secret;
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let secrets: Vec<isize> = INPUT.numbers();
    let mut map = vec![0; 19usize.pow(4)];
    let mut seen = vec![false; 19usize.pow(4)];

    let mut buf = [0; 4];
    for mut secret in secrets {
        let mut prev = secret % 10;
        for i in 0..2000 {
            evolve(&mut secret);

            let digit = secret % 10;
            buf[i % 4] = digit - prev;
            prev = digit;

            if i > 2 {
                let k = ((buf[(i + 1) % 4] + 9) * 19 * 19 * 19
                    + (buf[(i + 2) % 4] + 9) * 19 * 19
                    + (buf[(i + 3) % 4] + 9) * 19
                    + (buf[i % 4] + 9)) as usize;

                if !seen[k] {
                    seen[k] = true;
                    map[k] += digit;
                }
            }
        }

        for v in &mut seen {
            *v = false;
        }
    }

    map.iter().max().unwrap().solution()
}

fn evolve(secret: &mut isize) {
    *secret ^= *secret << 6;
    *secret &= 0xFFFFFF;
    *secret ^= *secret >> 5;
    *secret &= 0xFFFFFF;
    *secret ^= *secret << 11;
    *secret &= 0xFFFFFF;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (17724064040 as isize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (1998 as isize).solution());
    }
}
