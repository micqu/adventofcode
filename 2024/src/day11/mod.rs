use crate::utils::solution::{Solution, IntoSolution};

pub const TITLE: &str = "?!";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    None
}

pub fn part2() -> Option<Solution> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (472 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (969 as usize).solution());
    }
}