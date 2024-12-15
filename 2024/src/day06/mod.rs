use std::collections::HashSet;

use crate::utils::{
    grid::{grid::Grid, iterators::ADJ_FOUR},
    points::point2d::Point2d,
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Guard Gallivant";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (start, map) = parse();
    let mut seen = Grid::from(0, map.width, map.height);
    seen[(start.x as usize, start.y as usize)] = 1;

    let mut pos = start;
    let mut dir = 1;
    let mut d = ADJ_FOUR[dir];
    let mut next = (pos.x + d.0, pos.y + d.1);

    while let Some(n) = map.contains_point(&next) {
        if map[n] == b'#' {
            dir = (dir + 3) % 4;
            d = ADJ_FOUR[dir];
        } else {
            seen[n] = 1;
            pos.x += d.0;
            pos.y += d.1;
        }

        next = (pos.x + d.0, pos.y + d.1);
    }

    seen.data.iter().sum::<usize>().solution()
}

pub fn part2() -> Option<Solution> {
    let (mut pos, map) = parse();
    let mut dir: usize = 1;
    let mut d = ADJ_FOUR[dir];
    let mut next = (pos.x + d.0, pos.y + d.1);
    let mut seen = Grid::from([false; 4], map.width, map.height);
    let mut result = HashSet::<(isize, isize)>::new();

    while let Some(n) = map.contains_point(&next) {
        if seen[n].iter().all(|x| !*x) && check(pos, dir, next, &map, &seen) {
            result.insert(next);
        }

        if map[n] == b'#' {
            dir = (dir + 3) % 4;
            d = ADJ_FOUR[dir];
        } else {
            pos.x += d.0;
            pos.y += d.1;
        }

        seen[pos][dir] = true;
        next = (pos.x + d.0, pos.y + d.1);
    }

    result.len().solution()
}

fn check(
    mut pos: Point2d,
    mut dir: usize,
    obj: (isize, isize),
    map: &Grid<u8>,
    seen: &Grid<[bool; 4]>,
) -> bool {
    let mut d = ADJ_FOUR[dir];
    let mut next = (pos.x + d.0, pos.y + d.1);
    let mut inner_seen = Grid::from([false; 4], map.width, map.height);
    inner_seen[pos][dir] = true;

    while let Some(n) = map.contains_point(&next) {
        if map[n] == b'#' || next == obj {
            dir = (dir + 3) % 4;
            d = ADJ_FOUR[dir];
        } else {
            pos.x += d.0;
            pos.y += d.1;
        }

        if seen[pos][dir] || inner_seen[pos][dir] {
            return true;
        }

        inner_seen[pos][dir] = true;
        next = (pos.x + d.0, pos.y + d.1);
    }

    false
}

fn parse() -> (Point2d, Grid<u8>) {
    let mut start = Point2d::new(0, 0);
    let mut h: usize = 0;
    let k = INPUT
        .lines()
        .inspect(|x| {
            if let Some(s) = x.find('^') {
                start.x = s as isize;
                start.y = h as isize;
            }
            h += 1;
        })
        .map(|x| x.bytes())
        .flatten()
        .collect();

    (start, Grid::from_vec_height(k, h))
}

// fn parse2() -> ((isize, isize), Vec2d<[(Point, usize); 4]>) {
//     let w = INPUT.lines().next().unwrap().len();
//     let h = INPUT.lines().count();
//     let mut start: (isize, isize) = (0, 0);
//     let mut map = Vec2d::<[(Point, usize); 4]>::new(Vec::new(), w, h);

//     let mut y = 0;
//     for line in INPUT.lines() {
//         let mut x = 0;
//         let mut line = line.bytes();
//         while let Some(c) = line.next() {
//             if c == b'#' {

//             }
//             x += 1;
//         }
//         y += 1;
//     }

//     (start, map)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (4939 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (1434 as usize).solution());
    }
}
