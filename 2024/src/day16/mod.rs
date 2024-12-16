use std::collections::{BinaryHeap, HashMap, VecDeque};

use itertools::Itertools;

use crate::utils::{
    grid::{grid::Grid, iterators::ADJ_FOUR},
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
    let (min_cost, mut costs) = solve2(&start, &end, &map).unwrap();
    let tiles = backwards(&start, &end, min_cost, &map, &mut costs);
    tiles.data.iter().filter(|x| **x).count().solution()

    // let (start, end, map) = parse();
    // let mut seen = map.same_size_with(false);
    // let mut tiles = map.same_size_with(false);
    // tiles[end] = true;

    // let best = solve(&start, &end, &map).unwrap();
    // let mut costs = map.same_size_with([isize::MAX; 4]);
    // solve2(&start, &end, 0, 0, best, &map, &mut seen, &mut tiles, &mut costs);
    // tiles.data.iter().filter(|x| **x).count().solution()
}

fn solve(p: &Point2d, end: &Point2d, map: &Grid<u8>) -> Option<isize> {
    let mut costs = map.same_size_with(isize::MAX);
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
            if map[n] == b'#' || (nd + 2 % 4) == nd {
                continue;
            }

            let d = u.dir.abs_diff(nd) % 2;
            let c = u.cost + (d * 1000) as isize + 1;
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
    start: &Point2d,
    end: &Point2d,
    map: &Grid<u8>,
) -> Option<(isize, Grid<[isize; 4]>)> {
    let mut q = BinaryHeap::<State>::new();
    q.push(State {
        pos: *start,
        cost: 0,
        dir: 0,
    });

    let mut costs = map.same_size_with([isize::MAX; 4]);
    costs[start][0] = 0;
    
    let mut min_cost = isize::MAX;
    while let Some(u) = q.pop() {
        if u.pos == *end {
            min_cost = min_cost.min(u.cost);
            continue;
        }

        if u.cost > costs[u.pos][u.dir] {
            continue;
        }

        let next = [
            (u.pos, (u.dir + 1) % 4, u.cost + 1000),
            (u.pos, (u.dir + 3) % 4, u.cost + 1000),
            (u.pos.dir4(u.dir), u.dir, u.cost + 1),
        ];

        for (n, nd, nc) in next {
            if map[n] != b'#' && nc < costs[n][nd] {
                costs[n][nd] = nc;
                q.push(State {
                    pos: n,
                    cost: nc,
                    dir: nd,
                });
            }
        }
    }

    (min_cost != isize::MAX).then_some((min_cost, costs))
}

fn backwards(
    start: &Point2d,
    end: &Point2d,
    min_cost: isize,
    map: &Grid<u8>,
    costs: &mut Grid<[isize; 4]>,
) -> Grid<bool> {
    let mut tiles = map.same_size_with(false);
    let mut q = VecDeque::new();
    for d in 0..4 {
        if costs[end][d] == min_cost {
            q.push_back((*end, d, min_cost));
        }
    }

    while let Some((pos, dir, cost)) = q.pop_front() {
        tiles[pos] = true;

        if pos == *start {
            continue;
        }

        let next = [
            (pos, (dir + 1) % 4, cost - 1000),
            (pos, (dir + 3) % 4, cost - 1000),
            (pos.dir4((dir + 2) % 4), dir, cost - 1),
        ];

        for (n, nd, nc) in next {
            if nc == costs[n][nd] {
                q.push_back((n, nd, nc));
                costs[n][nd] = isize::MAX;
            }
        }
    }

    tiles
}

// fn solve2(
//     p: &Point2d,
//     end: &Point2d,
//     cost: isize,
//     dir: isize,
//     best: isize,
//     map: &Grid<u8>,
//     seen: &mut Grid<bool>,
//     tiles: &mut Grid<bool>,
//     costs: &mut Grid<[isize; 4]>,
// ) -> bool {
//     if p == end {
//         return true;
//     }

//     seen[p] = true;
//     let mut found = false;
//     for (nx, ny, nd) in map.four_connected_point2d(p) {
//         let n = Point2d::new(nx as isize, ny as isize);

//         if map[n] == b'#' || seen[n] || (nd + 2 % 4) == dir {
//             continue;
//         }

//         let d = dir.abs_diff(nd) % 2;
//         let c = cost + d * 1000 + 1;
//         if c <= best && c <= costs[n][nd] {
//             if solve2(&n, end, c, nd, best, map, seen, tiles, costs) {
//                 tiles[p] = true;
//                 found = true;
//             }
//             costs[n][nd] = c;
//         }
//     }
//     seen[p] = false;
//     found
// }

#[derive(Debug, Eq, PartialEq)]
struct State {
    pos: Point2d,
    cost: isize,
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
        assert_eq!(super::part1(), (93436 as isize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (486 as usize).solution());
    }
}
