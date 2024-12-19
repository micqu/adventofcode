use std::{collections::BinaryHeap, iter::from_fn};

use itertools::Itertools;
use num::traits::ops::bytes;

use crate::utils::{
    grid::grid::Grid,
    points::point2d::Point2d,
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "RAM Run";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let bytes = parse();

    let (width, height) = (71, 71);
    let mut map = Grid::new_with(false, width, height);
    for p in bytes.iter().take(1024) {
        map[p] = true;
    }

    let start = Point2d::new(0, 0);
    let end = Point2d::new(width as isize - 1, height as isize - 1);
    solve(&start, &end, &map).unwrap().solution()
}

pub fn part2() -> Option<Solution> {
    let bytes = parse();

    let (width, height) = (71, 71);
    let mut map = Grid::new_with(false, width, height);
    for p in &bytes {
        map[p] = true;
    }
    let mut visited = map.same_size_with(false);

    let start = Point2d::new(0, 0);
    let end = Point2d::new(width as isize - 1, height as isize - 1);
    solve2(&end, &start, &mut map, &mut visited);

    // Backwards recursive search.
    for (i, p) in bytes.iter().enumerate().rev() {
        map[p] = false;
        for (n, _) in map.four_connected_point2d(&p) {
            if visited[n] && solve2(&p, &start, &mut map, &mut visited) {
                return format!("{},{}", bytes[i].x, bytes[i].y).solution();
            }
        }
    }

    None
}

fn solve(start: &Point2d, end: &Point2d, map: &Grid<bool>) -> Option<usize> {
    let mut costs = map.same_size_with(usize::MAX);
    costs[start] = 0;

    let mut q = BinaryHeap::<State>::new();
    q.push(State {
        steps: 0,
        pos: *start,
    });

    while let Some(u) = q.pop() {
        if u.pos == *end {
            return Some(u.steps);
        }

        for (n, _) in map.four_connected_point2d(&u.pos) {
            let c = u.steps + 1;
            if !map[n] && c < costs[n] {
                costs[n] = c;
                q.push(State { steps: c, pos: n });
            }
        }
    }

    None
}

fn solve2(p: &Point2d, end: &Point2d, map: &mut Grid<bool>, visited: &mut Grid<bool>) -> bool {
    if p == end {
        return true;
    }

    visited[p] = true;
    for (n, _) in map.four_connected_point2d(p) {
        if !map[n] && !visited[n] {
            if solve2(&n, end, map, visited) {
                return true;
            }
        }
    }

    false
}

// Binary search.
// fn solve2(
//     start: &Point2d,
//     end: &Point2d,
//     map: &mut Grid<bool>,
//     bytes: &Vec<Point2d>,
//     i: usize,
// ) -> Option<Point2d> {
//     let mut left = i;
//     let mut right = bytes.len() - 1;

//     while left <= right {
//         let m = (left + right) / 2;

//         for i in left..=m {
//             map[bytes[i]] = true;
//         }

//         if let Some(_) = solve(start, end, map) {
//             left = m + 1;
//         } else {
//             for i in left..=m {
//                 map[bytes[i]] = false;
//             }
//             right = m - 1;
//         }

//         if left == right {
//             map[bytes[left + 1]] = true;
//             if let Some(_) = solve(start, end, map) {
//                 return Some(bytes[left]);
//             } else {
//                 return Some(bytes[left + 1]);
//             }
//         }
//     }

//     None
// }

#[derive(Debug, Eq, PartialEq)]
struct State {
    steps: usize,
    pos: Point2d,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}

fn parse() -> Vec<Point2d> {
    let mut bytes = INPUT.bytes();
    from_fn(|| Point2d::try_parse(&mut bytes)).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (226 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), "60,46".solution());
    }
}
