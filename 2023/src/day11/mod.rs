use itertools::Itertools;

use crate::utils::solution::{IntoSolution, Solution};

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
    let mut ps = Vec::<(isize, isize)>::new();
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if m[i][j] == b'#' {
                ps.push((j as isize, i as isize));
            }
        }
    }

    let c = find_empty(ps.iter().map(|p| p.0));
    let r = find_empty(ps.iter().map(|p| p.1));

    for (px, py) in ps.iter_mut() {
        *px += c.iter().filter(|x| *px > **x).count() as isize * dist;
        *py += r.iter().filter(|y| *py > **y).count() as isize * dist;
    }

    ps.iter()
        .tuple_combinations()
        .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs())
        .sum()
}

fn find_empty<I>(galaxies: I) -> Vec<isize>
where
    I: Iterator<Item = isize>,
{
    let mut empty = Vec::<isize>::new();
    let mut l = 0;
    for x in galaxies.sorted_unstable().dedup() {
        for c in l..x {
            empty.push(c);
        }
        l = x + 1;
    }

    empty
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