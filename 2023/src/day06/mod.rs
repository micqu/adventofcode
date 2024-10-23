use crate::utils::{parse_u64, solution::{IntoSolution, Solution}, Parsable};

pub const TITLE: &str = "Wait For It";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut lines = INPUT.lines();
    let mut a = lines.next().unwrap().bytes();//.chars();
    let mut b = lines.next().unwrap().bytes();
    let mut s = 1;
    while let Some(time) = a.next_number() {
        if let Some(dist) = b.next_number() {
            s *= solve(time, dist);
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let mut lines = INPUT.lines();
    let mut a = lines.next().unwrap().chars().filter(|x| x.is_ascii_digit());
    let mut b = lines.next().unwrap().chars().filter(|x| x.is_ascii_digit());
    let time = parse_u64(&mut a).unwrap();
    let dist = parse_u64(&mut b).unwrap();
    
    solve(time, dist).solution()
}

fn solve(time: u64, dist: u64) -> u64 {
    let d = (time * time - 4 * (dist + 1)) as f64;
    let d = d.sqrt();
    let mut low = (time as f64 - d) / 2.0;
    let mut high = (time as f64 + d) / 2.0;

    if low.fract() > f64::EPSILON {
        low = low.ceil();
    }

    if high.fract() < f64::EPSILON {
        high = high.floor();
    }
    
    (high - low) as u64 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::U64(a) => assert_eq!(a, 4568778),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::U64(a) => assert_eq!(a, 28973936),
            _ => panic!(),
        }
    }
}