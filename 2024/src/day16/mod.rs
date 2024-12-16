use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;

use crate::utils::{
    grid::grid::Grid,
    points::point2d::Point2d,
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Reindeer Maze";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (start, end, map) = parse();
    solve(&start, &end, &map).unwrap().solution()
}

pub fn part2() -> Option<Solution> {
    let (start, end, map) = parse();
    let mut seen = map.same_size_with(false);
    let mut tiles = map.same_size_with(false);
    tiles[end] = true;

    let best = solve(&start, &end, &map).unwrap();
    let mut costs = map.same_size_with([usize::MAX; 4]);
    solve2(
        &start, &end, 0, 0, best, &map, &mut seen, &mut tiles, &mut costs,
    );

    tiles.data.iter().filter(|x| **x).count().solution()
}

fn solve(p: &Point2d, end: &Point2d, map: &Grid<u8>) -> Option<usize> {
    let mut costs = map.same_size_with(usize::MAX);
    costs[*p] = 0;

    let mut q = BinaryHeap::<State>::new();
    q.push(State {
        pos: *p,
        cost: 0,
        dir: 0,
    });

    while let Some(u) = q.pop() {
        if u.pos == *end {
            return Some(u.cost);
        }

        if u.cost > costs[u.pos] {
            continue;
        }

        for (nx, ny, nd) in map.four_connected_point2d(&u.pos) {
            let n = Point2d::new(nx as isize, ny as isize);
            if map[n] == b'#' {
                continue;
            }

            if (nd + 2 % 4) == nd {
                continue;
            }

            let d = u.dir.abs_diff(nd) % 2;
            let c = u.cost + d * 1000 + 1;
            if c < costs[n] {
                costs[n] = c;
                q.push(State {
                    pos: n,
                    cost: c,
                    dir: nd,
                });
            }
        }
    }
    None
}

fn solve2(
    p: &Point2d,
    end: &Point2d,
    cost: usize,
    dir: usize,
    best: usize,
    map: &Grid<u8>,
    seen: &mut Grid<bool>,
    tiles: &mut Grid<bool>,
    costs: &mut Grid<[usize; 4]>,
) -> bool {
    if p == end {
        return true;
    }

    seen[p] = true;
    let mut found = false;
    for (nx, ny, nd) in map.four_connected_point2d(p) {
        let n = Point2d::new(nx as isize, ny as isize);

        if map[n] == b'#' || seen[n] {
            continue;
        }

        if (nd + 2 % 4) == dir {
            continue;
        }

        let d = dir.abs_diff(nd) % 2;
        let c = cost + d * 1000 + 1;
        if c <= best && c <= costs[n][nd] {
            if solve2(&n, end, c, nd, best, map, seen, tiles, costs) {
                tiles[p] = true;
                found = true;
            }
            costs[n][nd] = c;
        }
    }
    seen[p] = false;
    found
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    pos: Point2d,
    cost: usize,
    dir: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse() -> (Point2d, Point2d, Grid<u8>) {
    let mut start = Point2d::new(0, 0);
    let mut end = Point2d::new(0, 0);
    let mut h: usize = 0;
    let mut lines = INPUT.lines();
    let mut m = Vec::new();
    while let Some(line) = lines.next() {
        if let Some(s) = line.find('S') {
            start.x = s as isize;
            start.y = h as isize;
        } else if let Some(e) = line.find('E') {
            end.x = e as isize;
            end.y = h as isize;
        }
        m.append(&mut line.bytes().collect_vec());
        h += 1;
    }

    (start, end, Grid::from_vec_height(m, h))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (93436 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (486 as usize).solution());
    }
}
