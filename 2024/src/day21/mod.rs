use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::utils::{
    grid::grid::Grid,
    points::point2d::Point2d,
    solution::{IntoSolution, Solution},
    ToDigit,
};

pub const TITLE: &str = "Keypad Conundrum";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut codes = parse();

    let start = Point2d::new(0, 0);
    let keypad = vec![
        Point2d::new(-1, 0),
        Point2d::new(-2, -1),
        Point2d::new(-1, -1),
        Point2d::new(0, -1),
        Point2d::new(-2, -2),
        Point2d::new(-1, -2),
        Point2d::new(0, -2),
        Point2d::new(-2, -3),
        Point2d::new(-1, -3),
        Point2d::new(0, -3),
    ];

    let point_map = |b: &u8| {
        if *b == b'A' {
            start
        } else {
            keypad[(b - b'0') as usize]
        }
    };

    let mut s = 0;
    let mut cache = HashMap::new();
    for code in &mut codes {
        let numeric_part = get_numeric(&code);
        code.insert(0, b'A');

        let len: usize = code
            .iter()
            .map(point_map)
            .tuple_windows()
            .map(|(a, b)| numeric(&a, &b, &start, 2, &mut cache))
            .sum();

        s += numeric_part * len;
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let mut codes = parse();

    let start = Point2d::new(0, 0);
    let keypad = vec![
        Point2d::new(-1, 0),
        Point2d::new(-2, -1),
        Point2d::new(-1, -1),
        Point2d::new(0, -1),
        Point2d::new(-2, -2),
        Point2d::new(-1, -2),
        Point2d::new(0, -2),
        Point2d::new(-2, -3),
        Point2d::new(-1, -3),
        Point2d::new(0, -3),
    ];

    let point_map = |b: &u8| {
        if *b == b'A' {
            start
        } else {
            keypad[(b - b'0') as usize]
        }
    };

    let mut s = 0;
    let mut cache = HashMap::new();
    for code in &mut codes {
        let numeric_part = get_numeric(&code);
        code.insert(0, b'A');

        let len: usize = code
            .iter()
            .map(point_map)
            .tuple_windows()
            .map(|(a, b)| numeric(&a, &b, &start, 25, &mut cache))
            .sum();

        s += numeric_part * len;
    }

    s.solution()
}

fn numeric(
    a: &Point2d,
    b: &Point2d,
    prev: &Point2d,
    l: usize,
    cache: &mut HashMap<(Point2d, Point2d, Point2d, usize), usize>,
) -> usize {
    let start = Point2d::new(0, 0);
    if *a == *b {
        return directional_cached(prev, &start, &start, l, cache);
    }

    let mut cost = usize::MAX;
    let d = (b - a).signum();
    if d.x != 0 {
        let left = Point2d::new(-2, 1);
        let right = Point2d::new(0, 1);
        let empty = Point2d::new(-2, 0);
        let c = if d.x == 1 { right } else { left };
        let n = Point2d::new(a.x + d.x, a.y);
        if n != empty {
            cost = cost.min(
                directional_cached(&prev, &c, &start, l, cache) + numeric(&n, b, &c, l, cache),
            );
        }
    }

    if d.y != 0 {
        let up = Point2d::new(-1, 0);
        let down = Point2d::new(-1, 1);
        let empty = Point2d::new(-2, 0);
        let c = if d.y == 1 { down } else { up };
        let n = Point2d::new(a.x, a.y + d.y);
        if n != empty {
            cost = cost.min(
                directional_cached(&prev, &c, &start, l, cache) + numeric(&n, b, &c, l, cache),
            );
        }
    }

    cost
}

fn directional_cached(
    a: &Point2d,
    b: &Point2d,
    prev: &Point2d,
    l: usize,
    cache: &mut HashMap<(Point2d, Point2d, Point2d, usize), usize>,
) -> usize {
    let key = (*a, *b, *prev, l);
    if cache.contains_key(&key) {
        return cache[&key];
    }

    let r = directional(a, b, prev, l, cache);
    cache.insert(key, r);
    r
}

fn directional(
    a: &Point2d,
    b: &Point2d,
    prev: &Point2d,
    l: usize,
    cache: &mut HashMap<(Point2d, Point2d, Point2d, usize), usize>,
) -> usize {
    if l == 0 {
        return 1;
    }

    let start = Point2d::new(0, 0);
    if *a == *b {
        return directional_cached(prev, &start, &start, l - 1, cache);
    }

    let mut cost = usize::MAX;
    let d = (b - a).signum();
    if d.x != 0 {
        let left = Point2d::new(-2, 1);
        let right = Point2d::new(0, 1);
        let empty = Point2d::new(-2, 0);
        let c = if d.x == 1 { right } else { left };
        let n = Point2d::new(a.x + d.x, a.y);
        if n != empty {
            cost = cost.min(
                directional_cached(&prev, &c, &start, l - 1, cache)
                    + directional_cached(&n, b, &c, l, cache),
            );
        }
    }

    if d.y != 0 {
        let up = Point2d::new(-1, 0);
        let down = Point2d::new(-1, 1);
        let empty = Point2d::new(-2, 0);
        let c = if d.y == 1 { down } else { up };
        let n = Point2d::new(a.x, a.y + d.y);
        if n != empty {
            cost = cost.min(
                directional_cached(&prev, &c, &start, l - 1, cache)
                    + directional_cached(&n, b, &c, l, cache),
            );
        }
    }

    cost
}

fn get_numeric(c: &Vec<u8>) -> usize {
    let mut n: usize = 0;
    let mut o = c.iter();
    while let Some(&d) = o.next() {
        if let Some(u) = d.to_digit() {
            n = n * 10 + u as usize;
        } else {
            break;
        }
    }
    n
}

fn parse() -> Vec<Vec<u8>> {
    INPUT.lines().map(|x| x.bytes().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (205160 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (252473394928452 as usize).solution());
    }
}
