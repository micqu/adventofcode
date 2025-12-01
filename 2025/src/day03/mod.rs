use crate::utils::{solution::{IntoSolution, Solution}, Parsable};

pub const TITLE: &str = "";
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
        assert_eq!(super::part1(), (2196996 as u32).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (23655822 as u32).solution());
    }
}
