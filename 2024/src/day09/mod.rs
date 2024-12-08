use crate::utils::{point::Point, solution::{IntoSolution, Solution}};

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
        let result = super::part1().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 54916),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 54728),
            _ => panic!(),
        }
    }
}