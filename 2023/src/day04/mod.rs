use crate::utils::solution::{Solution, IntoSolution};

const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    INPUT.lines().map(|card| {
        let mut c = 1;
        let mut dupe: u128 = 0;
        if let Some((_, numbers)) = card.split_once(':') {
            let mut chs = numbers.chars();
            let mut n = 0;
            while let Some(ch) = chs.next() {
                if let Some(digit) = ch.to_digit(10) {
                    n = n * 10 + digit;
                } else if n != 0 {
                    if (1 << n) & dupe > 0 {
                        c *= 2;
                    }
                    
                    dupe |= 1 << n;
                    n = 0;
                }
            }

            if n != 0 && (1 << n) & dupe > 0 {
                c *= 2;
            }
        }
        
        c >> 1
    }).sum::<u32>().solution()
}

pub fn part2() -> Option<Solution> {
    let mut count = vec![1; 213];
    INPUT.lines().enumerate().map(|(i, card)| {
        let mut c = 0;
        let mut dupe: u128 = 0;
        if let Some((_, numbers)) = card.split_once(':') {
            let mut chs = numbers.chars();
            let mut n = 0;
            while let Some(ch) = chs.next() {
                if let Some(digit) = ch.to_digit(10) {
                    n = n * 10 + digit;
                } else if n != 0 {
                    if (1 << n) & dupe > 0 {
                        c += 1;
                    }
                    
                    dupe |= 1 << n;
                    n = 0;
                }
            }

            if n != 0 && (1 << n) & dupe > 0 {
                c += 1;
            }
        }
        
        for j in 0..c {
            count[i + j + 1] += count[i];
        }
        count[i]
    }).sum::<u32>().solution()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 26914),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 13080971),
            _ => panic!(),
        }
    }
}