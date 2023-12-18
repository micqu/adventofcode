use std::collections::BinaryHeap;

use crate::utils::{
    solution::{IntoSolution, Solution},
    vec2d::{Vec2d, ADJ_FOUR},
};

pub const TITLE: &str = "Clumsy Crucible";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let map = parse();
    let goal = |node: &Node| node.x == map.width - 1 && node.y == map.height - 1;
    solve((0, 0), &map, goal, 1, 3).solution()
}

pub fn part2() -> Option<Solution> {
    let map = parse();
    let goal = |node: &Node| node.x == map.width - 1 && node.y == map.height - 1;
    solve((0, 0), &map, goal, 4, 10).solution()
}

fn parse() -> Vec2d<u8> {
    let mut map = Vec::<u8>::new();
    let mut w = 0;
    for (i, byte) in INPUT.bytes().enumerate() {
        if byte != b'\n' {
            map.push(byte - b'0');
        } else if w == 0 {
            w = i;
        }
    }
    Vec2d::from_vec(map, w)
}

fn solve(
    start: (usize, usize),
    map: &Vec2d<u8>,
    end: impl Fn(&Node) -> bool,
    min: usize,
    max: usize,
) -> usize {
    let g = (map.width - 1, map.height - 1);
    let mut dists = [
        Vec2d::from_vec(vec![usize::MAX; map.data.len()], map.width),
        Vec2d::from_vec(vec![usize::MAX; map.data.len()], map.width),
    ];
    let mut q = BinaryHeap::<Node>::new();

    for d in [0, 3] {
        q.push(Node::new(start.0, start.1, d));
    }
    
    while let Some(u) = q.pop() {
        if end(&u) {
            return u.heat;
        }

        for vd in 0..4 {
            if (vd + 2) % 4 == u.dir || u.dir == vd {
                continue;
            }

            let mut cost_acc = 0;
            for dist in 1..=max {
                let vx = u.x as isize + ADJ_FOUR[vd].0 * dist as isize;
                let vy = u.y as isize + ADJ_FOUR[vd].1 * dist as isize;

                if vx < 0 || vx >= map.width as isize || vy < 0 || vy >= map.height as isize {
                    continue;
                }

                let vx = vx as usize;
                let vy = vy as usize;

                cost_acc += map[(vx, vy)] as usize;

                if dist < min {
                    continue;
                }

                let c = u.heat + cost_acc;
                let k = vd % 2;
                if c < dists[k][(vx, vy)] {
                    dists[k][(vx, vy)] = c;
                    q.push(Node {
                        x: vx,
                        y: vy,
                        heat: c,
                        dir: vd,
                        est: c + g.0 - vx + g.1 - vy,
                    });
                }
            }
        }
    }

    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    heat: usize,
    dir: usize,
    est: usize,
}

impl Node {
    fn new(x: usize, y: usize, dir: usize) -> Self {
        Self { x, y, heat: 0, dir, est: 0 }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.est.cmp(&self.est)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 1099),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 1266),
            _ => panic!(),
        }
    }
}
