use std::collections::BinaryHeap;

use crate::utils::{
    grid::grid::Grid,
    points::point2d::Point2d,
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Race Condition";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let map = Grid::parse(INPUT);
    let start = map.find(b'S').unwrap();
    let end = map.find(b'E').unwrap();
    
    let costs = dfs(&start, &end, &map);
    find_cheats(2, &map, &costs).solution()
}

pub fn part2() -> Option<Solution> {
    let map = Grid::parse(INPUT);
    let start = map.find(b'S').unwrap();
    let end = map.find(b'E').unwrap();
    
    let costs = dfs(&start, &end, &map);
    find_cheats(20, &map, &costs).solution()
}

fn dfs(start: &Point2d, end: &Point2d, map: &Grid<u8>) -> Grid<isize> {
    let mut costs = map.same_size_with(isize::MAX);
    costs[start] = 0;

    let mut q = Vec::<(Point2d, isize)>::new();
    q.push((*start, 0));

    while let Some(u) = q.pop() {
        if u.0 == *end {
            break;
        }

        for (n, _) in map.four_connected_point2d(&u.0) {
            if map[n] != b'#' {
                let c = u.1 + 1;
                if c < costs[n] {
                    costs[n] = c;
                    q.push((n, c));
                }
            }
        }
    }

    costs
}

fn find_cheats(max_cheat_len: isize, map: &Grid<u8>, costs: &Grid<isize>) -> usize {
    let mut s: usize = 0;
    for p in map.positions() {
        if map[p] == b'#' {
            continue;
        }

        for i in -max_cheat_len..=max_cheat_len {
            for j in (i.abs() - max_cheat_len)..=(max_cheat_len - i.abs()) {
                let n = Point2d::new(p.0 as isize + i, p.1 as isize + j);
                if map.contains_point2d(&n) && map[n] != b'#' {
                    let len = i.abs() + j.abs();
                    if costs[p] - costs[n] - len >= 100 {
                        s += 1;
                    }
                }
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (1346 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (985482 as usize).solution());
    }
}
