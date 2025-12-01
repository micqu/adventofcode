use num::Integer;

use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Secret Entrance";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut pos: i32 = 50;
    let mut zeros = 0;

    INPUT.lines().for_each(|line| {
        let mut bytes = line.bytes();
        let dir = bytes.next().unwrap();
        let rot: i32 = bytes.next_number().unwrap();

        let d = if dir == b'L' { -1 } else { 1 };
        pos = (pos + rot * d).rem_euclid(100);

        if pos == 0 {
            zeros += 1;
        }
    });

    zeros.solution()
}

pub fn part2() -> Option<Solution> {
    let mut pos: i32 = 50;
    let mut zeros = 0;

    INPUT.lines().for_each(|line| {
        let mut bytes = line.bytes();
        let dir = bytes.next().unwrap();
        let rot: i32 = bytes.next_number().unwrap();

        zeros += rot / 100;
        let d = if dir == b'L' { -1 } else { 1 };
        let t = (pos + rot * d).rem_euclid(100);

        if pos != 0 {
            if dir == b'L' {
                if t > pos || t == 0 {
                    zeros += 1;
                }
            } else if pos > t || t == 0 {
                zeros += 1;
            }
        }

        pos = t;
    });

    zeros.solution()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (1071 as i32).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (6700 as i32).solution());
    }
}
