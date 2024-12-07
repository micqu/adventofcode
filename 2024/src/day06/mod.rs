use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::{
    solution::{IntoSolution, Solution},
    vec2d::Vec2d,
};

pub const TITLE: &str = "Guard Gallivant";
const INPUT: &'static str = include_str!("input.txt");

const DIR_FOUR: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

pub fn part1() -> Option<Solution> {
    let (start, map) = parse();
    let mut seen = Vec2d::new(vec![0; map.width * map.height], map.width, map.height);
    seen[(start.0 as usize, start.1 as usize)] = 1;

    let mut pos = start;
    let mut dir = 1;
    let mut d = DIR_FOUR[dir];
    let mut next = (pos.0 + d.0, pos.1 + d.1);

    while let Some(n) = map.contains(&next) {
        if map[n] == b'#' {
            dir = (dir + 3) % 4;
            d = DIR_FOUR[dir];
        } else {
            seen[n] = 1;
            pos.0 += d.0;
            pos.1 += d.1;
        }

        next = (pos.0 as isize + d.0, pos.1 as isize + d.1);
    }

    seen.data.iter().sum::<usize>().solution()
}

pub fn part2() -> Option<Solution> {
    let (mut pos, map) = parse();
    let mut dir: usize = 1;
    let mut d = DIR_FOUR[dir];
    let mut next = (pos.0 + d.0, pos.1 + d.1);
    let mut seen = Vec2d::new(vec![[false; 4]; map.width * map.height], map.width, map.height);
    let mut result = HashSet::<(isize, isize)>::new();
    
    while let Some(n) = map.contains(&next) {
        if seen[n].iter().all(|x| !*x) && check(pos, dir, next, &map, &seen) {
            result.insert(next);
        }
        
        if map[n] == b'#' {
            dir = (dir + 3) % 4;
            d = DIR_FOUR[dir];
        } else {
            pos.0 += d.0;
            pos.1 += d.1;
        }

        seen[(pos.0 as usize, pos.1 as usize)][dir] = true;
        next = (pos.0 + d.0, pos.1 + d.1);
    }

    result.len().solution()
}

fn check(
    mut pos: (isize, isize),
    mut dir: usize,
    obj: (isize, isize),
    map: &Vec2d<u8>,
    seen: &Vec2d::<[bool; 4]>,
) -> bool {
    let mut d = DIR_FOUR[dir];
    let mut next = (pos.0 + d.0, pos.1 + d.1);
    let mut inner_seen = Vec2d::new(vec![[false; 4]; map.width * map.height], map.width, map.height);
    inner_seen[(pos.0 as usize, pos.1 as usize)][dir] = true;

    while let Some(n) = map.contains(&next) {
        if map[n] == b'#' || next == obj {
            dir = (dir + 3) % 4;
            d = DIR_FOUR[dir];
        } else {
            pos.0 += d.0;
            pos.1 += d.1;
        }

        let c = (pos.0 as usize, pos.1 as usize);
        if seen[c][dir] || inner_seen[c][dir] {
            return true;
        }
        
        inner_seen[c][dir] = true;
        next = (pos.0 + d.0, pos.1 + d.1);
    }

    false
}

fn parse() -> ((isize, isize), Vec2d<u8>) {
    let mut start: (isize, isize) = (0, 0);
    let mut h: usize = 0;
    let k = INPUT
        .lines()
        .inspect(|x| {
            if let Some(s) = x.find('^') {
                start.0 = s as isize;
                start.1 = h as isize;
            }
            h += 1;
        })
        .map(|x| x.bytes())
        .flatten()
        .collect();

    (start, Vec2d::from_vec(k, h))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 4939),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 1434),
            _ => panic!(),
        }
    }
}
