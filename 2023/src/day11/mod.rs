use itertools::Itertools;

use crate::utils::solution::{IntoSolution, Solution};

pub const TITLE: &str = "Cosmic Expansion";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let m = parse();
    solve(&m, 1).solution()
}

pub fn part2() -> Option<Solution> {
    let m = parse();
    solve(&m, 1000000 - 1).solution()
}

fn parse() -> Vec<Vec<u8>> {
    INPUT
        .lines()
        .map(|line| line.bytes().collect_vec())
        .collect_vec()
}

fn solve(m: &Vec<Vec<u8>>, dist: isize) -> isize {
    let mut galaxies = Vec::<(isize, isize)>::new();
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if m[i][j] == b'#' {
                galaxies.push((j as isize, i as isize));
            }
        }
    }
    
    galaxies.sort_by_key(|a| a.0);
    let mut expanded = 0;
    let mut l = 0;
    for (x, _) in galaxies.iter_mut() {
        if *x != l {
            expanded += (*x - l - 1) * dist;
            l = *x;
        }

        *x += expanded;
    }

    galaxies.sort_by_key(|a| a.1);
    let mut expanded = 0;
    let mut l = 0;
    for (_, y) in galaxies.iter_mut() {
        if *y != l {
            expanded += (*y - l - 1) * dist;
            l = *y;
        }
        
        *y += expanded;
    }

    galaxies.iter()
        .tuple_combinations()
        .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Isize(a) => assert_eq!(a, 9556896),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Isize(a) => assert_eq!(a, 685038186836),
            _ => panic!(),
        }
    }
}