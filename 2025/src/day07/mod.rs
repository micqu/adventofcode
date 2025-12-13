use itertools::Itertools;

use crate::utils::{
    Parsable,
    grid::grid::Grid,
    points::point2d::Point2d,
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Laboratories";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut lines = INPUT.lines().step_by(2);

    let line = lines.next().unwrap();
    let mut filled = vec![false; line.len()];
    filled[line.find('S').unwrap() as usize] = true;

    let mut s = 0;
    for line in lines {
        for (i, b) in line.bytes().enumerate() {
            if b == b'^' {
                s += filled[i] as usize;
                filled[i - 1] = true;
                filled[i + 1] = true;
                filled[i] = false;
            }
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let mut lines = INPUT.lines().step_by(2);

    let line = lines.next().unwrap();
    let mut filled = vec![0_usize; line.len()];
    filled[line.find('S').unwrap() as usize] = 1;

    for line in lines {
        for (i, b) in line.bytes().enumerate() {
            if b == b'^' {
                filled[i - 1] += filled[i];
                filled[i + 1] += filled[i];
                filled[i] = 0;
            }
        }
    }

    filled.iter().sum::<usize>().solution()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (1678 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (357525737893560 as usize).solution());
    }
}
