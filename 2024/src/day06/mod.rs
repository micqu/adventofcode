use std::collections::HashSet;

use crate::utils::{
    grid::{grid::Grid, iterators::ADJ_FOUR},
    points::point2d::{self, Point2d},
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Guard Gallivant";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (start, map) = parse();
    let mut seen = map.same_size_with(0);
    seen[(start.x as usize, start.y as usize)] = 1;

    let mut pos = start;
    let mut dir = 1;
    let mut d = ADJ_FOUR[dir];
    let mut next = (pos.x + d.0, pos.y + d.1);

    while map.contains_point(&next) {
        if map[next] == b'#' {
            dir = (dir + 3) % 4;
            d = ADJ_FOUR[dir];
        } else {
            seen[next] = 1;
            pos.x += d.0;
            pos.y += d.1;
        }

        next = (pos.x + d.0, pos.y + d.1);
    }

    seen.data.iter().sum::<usize>().solution()
}

pub fn part2() -> Option<Solution> {
    // let (start, map, jump) = parse2();
    // dbg!(jump);
    // None
    let (mut pos, map) = parse();
    let mut dir: usize = 1;
    let mut d = ADJ_FOUR[dir];
    let mut next = (pos.x + d.0, pos.y + d.1);
    let mut seen = map.same_size_with([false; 4]);
    let mut result = HashSet::<(isize, isize)>::new();

    while map.contains_point(&next) {
        if seen[next].iter().all(|x| !*x) && check(pos, dir, next, &map, &seen) {
            result.insert(next);
        }

        if map[next] == b'#' {
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
    let mut inner_seen = map.same_size_with([false; 4]);
    inner_seen[pos][dir] = true;

    while map.contains_point(&next) {
        if map[next] == b'#' || next == obj {
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

fn parse2() -> (Point2d, Grid<u8>, Grid<[Option<Point2d>; 4]>) {
    let w = INPUT.lines().next().unwrap().len();
    let h = INPUT.lines().count();
    let mut map = Grid::new(Vec::new(), w, h);
    let mut jump: Grid<[Option<Point2d>; 4]> = Grid::new(Vec::new(), w, h);
    let mut start = Point2d::new(0, 0);

    let mut y = 0;
    for line in INPUT.lines() {
        let mut x = 0;
        let mut line = line.bytes();
        while let Some(c) = line.next() {
            if c == b'^' {
                start.x = x;
                start.y = y;
            }

            if c == b'#' {
                let current_point = Point2d::new(x, y);
                set(current_point, 2, Some(Point2d::new(x, y)), &map, &mut jump);
                set(current_point, 1, Some(Point2d::new(x, y)), &map, &mut jump);
            }

            map.data.push(c);
            x += 1;
        }

        y += 1;
    }

    for i in 0..w as isize {
        let u = Point2d::new(i, 0);
        let d = Point2d::new(i, h as isize - 1);
        set(u, 1, Some(u), &map, &mut jump);
        set(d, 3, Some(d), &map, &mut jump);
    }

    for j in 0..h as isize {
        let l = Point2d::new(0, j);
        let r = Point2d::new(w as isize - 1, j);
        set(l, 2, Some(l), &map, &mut jump);
        set(r, 0, Some(r), &map, &mut jump);
    }

    (start, map, jump)
}

fn set(p: Point2d, d: usize, value: Option<Point2d>, map: &Grid<u8>, jump: &mut Grid<[Option<Point2d>; 4]>) {
    let mut t = p;
    
    while map.contains_point2d(&t) {
        if map[t] == b'#' {
            break;
        }
        jump[t][d] = value;
        t.move_dir4(d);
    }
}

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
