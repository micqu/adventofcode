use std::collections::{HashMap, VecDeque};

use crate::utils::{
    solution::{IntoSolution, Solution},
    vec2d::Vec2d,
};

pub const TITLE: &str = "Step Counter";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (start, map) = parse();
    solve(start, &map, 64).solution()
}

pub fn part2() -> Option<Solution> {
    let (start, map) = parse();
    let mut f = Vec::<Point>::new();
    for i in 0..3 {
        let steps = 65 + map.width * i;
        let plots = solve(start, &map, steps);
        f.push(Point {
            x: steps as f64,
            y: plots as f64,
        });
    }
    interpolate(&f, 26501365.0).solution()
}

fn parse() -> ((isize, isize), Vec2d<u8>) {
    let mut map = Vec::<u8>::new();
    let mut start = (0, 0);
    let mut i = 0;
    let mut h = 0;
    for byte in INPUT.bytes() {
        if byte == b'\n' {
            h += 1;
            continue;
        }

        map.push(byte);

        if byte == b'S' {
            start.0 = i as isize;
            start.1 = h as isize;
        }
        i += 1;
    }
    h += 1;
    let w = map.len() / h;
    start.0 %= w as isize;
    (start, Vec2d::from_vec(map, w))
}

fn solve(start: (isize, isize), map: &Vec2d<u8>, max_steps: usize) -> usize {
    let mut seen = HashMap::<(isize, isize), usize>::new();
    let mut q = VecDeque::<State>::new();
    q.push_back(State {
        x: start.0,
        y: start.1,
        len: 0,
    });

    let mut s = 0;
    let f = max_steps % 2;
    while let Some(u) = q.pop_front() {
        if u.len > max_steps {
            break;
        }

        for (vx, vy, _) in map.four_connected_unbound(u.x, u.y) {
            if seen.contains_key(&(vx, vy)) {
                continue;
            }

            let nx = vx.rem_euclid(map.width as isize) as usize;
            let ny = vy.rem_euclid(map.height as isize) as usize;
            if map[(nx, ny)] != b'#' {
                let vl = u.len + 1;
                seen.insert((vx, vy), vl);
                q.push_back(State {
                    x: vx,
                    y: vy,
                    len: vl,
                });

                if f == vl % 2 {
                    s += 1;
                }
            }
        }
    }
    s
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    x: isize,
    y: isize,
    len: usize,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn interpolate(f: &Vec<Point>, x: f64) -> f64 {
    let n = f.len();
    let mut result = 0.0;
    for i in 0..n {
        let mut term = f[i].y;
        for j in 0..n {
            if i != j {
                term *= (x - f[j].x) / (f[i].x - f[j].x);
            }
        }
        result += term;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 3600),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::F64(a) => assert_eq!(a, 599763113936220.0),
            _ => panic!(),
        }
    }
}
