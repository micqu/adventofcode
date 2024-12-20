use std::collections::BinaryHeap;

use crate::utils::{
    grid::grid::Grid,
    points::point2d::Point2d,
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Race Condition";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (start, end, map) = parse();
    let (costs, prev) = bfs(&start, &end, &map);

    let mut path_map = map.same_size_with(false);
    let mut path = vec![end];
    let mut current = end;
    while let Some(u) = prev[current] {
        path_map[u] = true;
        path.push(u);
        current = u;
    }

    find_cheats(2, &path, &path_map, &costs).solution()
}

pub fn part2() -> Option<Solution> {
    let (start, end, map) = parse();
    let (costs, prev) = bfs(&start, &end, &map);

    let mut path_map = map.same_size_with(false);
    let mut path = vec![end];
    let mut current = end;
    while let Some(u) = prev[current] {
        path_map[u] = true;
        path.push(u);
        current = u;
    }

    find_cheats(20, &path, &path_map, &costs).solution()
}

fn bfs(start: &Point2d, end: &Point2d, map: &Grid<u8>) -> (Grid<isize>, Grid<Option<Point2d>>) {
    let mut prev = map.same_size_with(None);
    let mut costs = map.same_size_with(isize::MAX);
    costs[start] = 0;

    let mut q = Vec::<State>::new();
    q.push(State {
        pos: *start,
        time: 0,
    });

    while let Some(u) = q.pop() {
        if u.pos == *end {
            break;
        }

        for (n, _) in map.four_connected_point2d(&u.pos) {
            let c = u.time + 1;
            if map[n] != b'#' && c < costs[n] {
                costs[n] = c;
                prev[n] = Some(u.pos);
                q.push(State { pos: n, time: c });
            }
        }
    }

    (costs, prev)
}

fn find_cheats(
    max_cheat_len: usize,
    path: &Vec<Point2d>,
    path_map: &Grid<bool>,
    costs: &Grid<isize>,
) -> usize {
    let mut s: usize = 0;
    let max_cheat_len = max_cheat_len as isize;
    for u in path {
        for i in -max_cheat_len..=max_cheat_len {
            for j in -max_cheat_len..=max_cheat_len {
                let len = i.abs() + j.abs();
                if len <= max_cheat_len {
                    let n = Point2d::new(u.x + i, u.y + j);
                    if path_map.contains_point2d(&n)
                        && path_map[n]
                        && costs[u] - costs[n] - len >= 100
                    {
                        s += 1;
                    }
                }
            }
        }
    }
    s
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    pos: Point2d,
    time: isize,
}

fn parse() -> (Point2d, Point2d, Grid<u8>) {
    let mut start = Point2d::new(0, 0);
    let mut end = Point2d::new(0, 0);
    let mut h: usize = 0;
    let k = INPUT
        .lines()
        .inspect(|x| {
            if let Some(s) = x.find('S') {
                start.x = s as isize;
                start.y = h as isize;
            } else if let Some(s) = x.find('E') {
                end.x = s as isize;
                end.y = h as isize;
            }

            h += 1;
        })
        .map(|x| x.bytes())
        .flatten()
        .collect();

    (start, end, Grid::from_vec_height(k, h))
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
