use crate::utils::solution::{Solution, IntoSolution};

pub const TITLE: &str = "Trebuchet?!";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    INPUT.lines().map(|line| {
        let mut first = 0;
        let mut last = 0;

        for ch in line.chars() {
            if ch.is_ascii_digit() {
                first = ch.to_digit(10).unwrap();
                break;
            }
        }

        for ch in line.chars().rev() {
            if ch.is_ascii_digit() {
                last = ch.to_digit(10).unwrap();
                break;
            }
        }

        first * 10 + last
    }).sum::<u32>().solution()
}

pub fn part2() -> Option<Solution> {
    INPUT.lines().map(|line| {
        let mut first = 0;
        let mut last = 0;
        
        let mut chars = line.chars();
        loop {
            if let Some(o) = parse_digit_start(chars.as_str()) {
                first = o;
                break;
            }
            
            if let Some(ch) = chars.next() {
                if let Some(digit) = ch.to_digit(10) {
                    first = digit;
                    break;
                }
            } else {
                break;
            }
        }

        let mut chars = line.chars();
        loop {
            if let Some(o) = parse_digit_end(chars.as_str()) {
                last = o;
                break;
            }

            if let Some(ch) = chars.next_back() {
                if let Some(digit) = ch.to_digit(10) {
                    last = digit;
                    break;
                }
            } else {
                break;
            }
        }

        first * 10 + last
    }).sum::<u32>().solution()
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_digit_end(str: &str) -> Option<u32> {
    for (i, n) in NUMBERS.iter().enumerate() {
        if str.ends_with(n) {
            return Some(i as u32 + 1);
        }
    }
    None
}

fn parse_digit_start(str: &str) -> Option<u32> {
    for (i, n) in NUMBERS.iter().enumerate() {
        if str.starts_with(n) {
            return Some(i as u32 + 1);
        }
    }
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