use itertools::Itertools;

use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Mirage Maintenance";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    INPUT
        .lines()
        .map(|line| {
            let mut chs = line.bytes();
            let mut f = Vec::<Point>::new();
            let mut i = 0;
            while let Some(n) = Parsable::<i64>::next_number(&mut chs) {
                f.push(Point {
                    x: i as f64,
                    y: n as f64,
                });
                i += 1;
            }

            interpolate(&f, f.len() as f64)
        })
        .sum::<f64>()
        .round()
        .solution()

    // INPUT.lines().map(|line| {
    //     let mut chs = line.chars();
    //     let mut cs = Vec::<Vec<i64>>::new();
    //     cs.push(Vec::new());
    //     while let Some(n) = parse_i64(&mut chs) {
    //         cs[0].push(n);
    //     }
    //     solve(&mut cs)
    // }).sum::<i64>().solution()
}

pub fn part2() -> Option<Solution> {
    INPUT
        .lines()
        .map(|line| {
            let mut chs = line.bytes();

            let mut f = Vec::<Point>::new();
            let mut i = 0;
            while let Some(n) = Parsable::<i64>::next_number(&mut chs) {
                f.push(Point {
                    x: i as f64,
                    y: n as f64,
                });
                i += 1;
            }

            interpolate(&f, -1.)
        })
        .sum::<f64>()
        .round()
        .solution()

    // INPUT.lines().map(|line| {
    //     let mut chs = line.chars();
    //     let mut cs = Vec::<Vec<i64>>::new();
    //     cs.push(Vec::new());
    //     while let Some(n) = parse_i64(&mut chs) {
    //         cs[0].insert(0, n);
    //     }
    //     solve(&mut cs)
    // }).sum::<i64>().solution()
}

fn solve(cs: &mut Vec<Vec<i64>>) -> i64 {
    let mut i = 0;
    loop {
        if (&cs[i]).iter().all(|&x| x == 0) {
            return (0..cs.len()).rev().map(|j| cs[j].last().unwrap()).sum();
        }

        cs.push(cs[i].windows(2).map(|c| c[1] - c[0]).collect_vec());
        i += 1;
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// lagrange interpolation at x
fn interpolate(f: &Vec<Point>, x: f64) -> f64 {
    let n = f.len() as usize;
    let mut result = 0.0;
    for i in 0..n {
        let mut term = f[i].y;
        for j in 0..n {
            if i != j {
                term *= (x - f[j].x) / (f[i].x - f[j].x);
            }
        }
        result += term;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::F64(a) => assert_eq!(a, 1798691765.0),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::F64(a) => assert_eq!(a, 1104.0),
            _ => panic!(),
        }
    }
}
