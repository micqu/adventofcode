use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable, ToNumbers,
};

pub const TITLE: &str = "Red-Nosed Reports";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    INPUT
        .lines()
        .map(|line| check(&mut line.bytes()))
        .filter(|x| *x)
        .count()
        .solution()
}

pub fn part2() -> Option<Solution> {
    INPUT
        .lines()
        .map(|line| {
            let numbers: Vec<i16> = line.to_numbers();
            for n in 0..numbers.len() {
                let mut filtered = numbers
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != n)
                    .map(|(_, x)| x);
                if check2(&mut filtered) {
                    return true;
                }
            }
            false
        })
        .filter(|x| *x)
        .count()
        .solution()
}

fn check(bytes: &mut impl Iterator<Item = u8>) -> bool {
    let a: i16 = bytes.next_number().unwrap();
    let mut b: i16 = bytes.next_number().unwrap();
    let d = a.abs_diff(b);
    if d == 0 || d > 3 {
        return false;
    }

    let inc = b - a > 0;

    while let Some(n) = Parsable::<i16>::next_number(bytes) {
        let d = n - b;
        let s = d.abs();
        if s <= 0 || s > 3 || (d > 0 && !inc) || (d < 0 && inc) {
            return false;
        }
        b = n;
    }
    true
}

fn check2<'a>(bytes: &mut impl Iterator<Item = &'a i16>) -> bool {
    let a: i16 = *bytes.next().unwrap();
    let mut b: i16 = *bytes.next().unwrap();
    let d = a.abs_diff(b);
    if d == 0 || d > 3 {
        return false;
    }

    let inc = b - a > 0;

    while let Some(n) = bytes.next() {
        let d = n - b;
        let s = d.abs();
        if s <= 0 || s > 3 || (d > 0 && !inc) || (d < 0 && inc) {
            return false;
        }
        b = *n;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 269),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 337),
            _ => panic!(),
        }
    }
}
