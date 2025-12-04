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

    for i in 0..map.width {
        for j in 0..map.height {
            if map[(i, j)] != b'@' {
                continue;
            }

            if map
                .eight_connected_point((i, j))
                .map(|x| map[x] == b'@')
                .filter(|x| *x)
                .count()
                < 4
            {
                s += 1;
            }
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let mut map = parse();
    let mut s: usize = 0;
    let mut d = true;

    while d {
        d = false;

        for i in 0..map.width {
            for j in 0..map.height {
                if map[(i, j)] != b'@' {
                    continue;
                }

                if map
                    .eight_connected_point((i, j))
                    .map(|x| map[x] == b'@')
                    .filter(|x| *x)
                    .count()
                    < 4
                {
                    s += 1;
                    map[(i, j)] = b'.';
                    d = true;
                }
            }
        }
    }

    s.solution()
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
