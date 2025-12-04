use std::collections::{BinaryHeap, VecDeque};

use itertools::Itertools;

use crate::utils::{
    grid::grid::Grid,
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Printing Department";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let map = parse();
    let mut s: usize = 0;

    for u in map.positions() {
        if map[u] != b'@' {
            continue;
        }

        if count_paper(u, &map) < 4 {
            s += 1;
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let mut map = parse();
    let mut s: usize = 0;
    let mut q = VecDeque::<(usize, usize)>::new();

    for u in map.positions() {
        if map[u] != b'@' {
            continue;
        }

        if count_paper(u, &map) < 4 {
            s += clear(u, &mut q, &mut map);
        }
    }

    s.solution()
}

fn clear(u: (usize, usize), q: &mut VecDeque<(usize, usize)>, map: &mut Grid<u8>) -> usize {
    q.push_back(u);

    let mut s = 0;
    while let Some(u) = q.pop_front() {
        if map[u] != b'@' || count_paper(u, map) >= 4 {
            continue;
        }

        map[u] = 0;
        s += 1;

        for n in map.eight_connected_point(u) {
            if map[n] == b'@' {
                q.push_back(n);
            }
        }
    }

    s
}

fn count_paper(u: (usize, usize), map: &Grid<u8>) -> usize {
    map.eight_connected_point(u)
        .map(|x| map[x] == b'@')
        .filter(|x| *x)
        .count()
}

fn parse() -> Grid<u8> {
    let mut map = Vec::new();
    let mut h = 0;
    for line in INPUT.lines() {
        map.append(&mut line.bytes().collect_vec());
        h += 1;
    }

    Grid::from_vec_height(map, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (1424 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (8727 as usize).solution());
    }
}
