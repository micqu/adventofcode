use core::panic;
use std::iter::from_fn;

use itertools::Itertools;

use crate::utils::{
    grid::{grid::Grid, iterators::ADJ_FOUR},
    points::point2d::Point2d,
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Warehouse Woes";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (mut pos, mut map, moves) = parse();
    for m in moves {
        let n = Point2d::move_dir(&pos, m);
        if map.contains_point2d(&n) {
            match map[n] {
                b'.' => {
                    pos = n;
                }
                b'#' => {}
                b'O' => {
                    let mut t = n;
                    let mut c = map[t];
                    while c != b'#' {
                        if c == b'.' {
                            map[t] = b'O';
                            map[n] = b'.';
                            pos = n;
                            break;
                        }
                        t = Point2d::move_dir(&t, m);
                        c = map[t];
                    }
                }
                _ => panic!(),
            }
        }
    }

    map.positions()
        .filter(|&p| map[p] == b'O')
        .map(|p| p.0 + p.1 * 100)
        .sum::<usize>()
        .solution()
}

pub fn part2() -> Option<Solution> {
    let (mut pos, mut map, moves) = parse2();
    for m in moves {
        let n = Point2d::move_dir(&pos, m);
        if map.contains_point2d(&n) {
            match map[n] {
                b'.' => {
                    pos = n;
                }
                b'#' => {}
                b'[' | b']' => {
                    if can_push(&pos, &n, m, &mut map) {
                        push(&pos, &n, m, &mut map);
                        pos = n;
                    }
                }
                _ => panic!(),
            }
        }
    }

    map.positions()
        .filter(|&p| map[p] == b'[')
        .map(|p| p.0 + p.1 * 100)
        .sum::<usize>()
        .solution()
}

fn can_push(prev: &Point2d, c: &Point2d, m: usize, map: &mut Grid<u8>) -> bool {
    if map[c] == b'.' {
        return true;
    }

    if map[c] == b'#' {
        return false;
    }

    let next = Point2d::move_dir(c, m);
    if m % 2 == 1 {
        let other = if map[c] == b'[' {
            Point2d::move_dir(c, 0)
        } else {
            Point2d::move_dir(c, 2)
        };

        if *prev != other {
            return can_push(c, &other, m, map) && can_push(c, &next, m, map);
        }
    }
    can_push(c, &next, m, map)
}

fn push(prev: &Point2d, c: &Point2d, m: usize, map: &mut Grid<u8>) {
    if map[c] == b'.' || map[c] == b'#' {
        return;
    }

    let next = Point2d::move_dir(c, m);
    if m % 2 == 1 {
        let other = if map[c] == b'[' {
            Point2d::move_dir(c, 0)
        } else {
            Point2d::move_dir(c, 2)
        };

        if *prev != other {
            push(c, &other, m, map);
        }
    }
    push(c, &next, m, map);

    map[next] = map[c];
    map[c] = b'.';
}

fn parse() -> (Point2d, Grid<u8>, Vec<usize>) {
    let mut start = Point2d::new(0, 0);
    let mut h: usize = 0;
    let mut lines = INPUT.lines();
    let mut m = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        if let Some(s) = line.find('@') {
            start.x = s as isize;
            start.y = h as isize;
            m.append(
                &mut line
                    .bytes()
                    .map(|x| if x == b'@' { b'.' } else { x })
                    .collect_vec(),
            );
        } else {
            m.append(&mut line.bytes().collect_vec());
        }

        h += 1;
    }

    (start, Grid::from_vec_height(m, h), parse_moves(&mut lines))
}

fn parse2() -> (Point2d, Grid<u8>, Vec<usize>) {
    let mut start = Point2d::new(0, 0);
    let mut h: usize = 0;
    let mut lines = INPUT.lines();
    let mut m = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        for (i, b) in line.bytes().enumerate() {
            match b {
                b'.' => m.append(&mut vec![b'.', b'.']),
                b'@' => {
                    start.x = (i * 2) as isize;
                    start.y = h as isize;
                    m.append(&mut vec![b'.', b'.'])
                }
                b'#' => m.append(&mut vec![b'#', b'#']),
                b'O' => m.append(&mut vec![b'[', b']']),
                _ => panic!(),
            }
        }

        h += 1;
    }

    (start, Grid::from_vec_height(m, h), parse_moves(&mut lines))
}

fn parse_moves<T>(lines: &mut T) -> Vec<usize>
where
    T: IntoIterator + Iterator,
    T: Iterator<Item = &'static str>,
{
    let mut moves = Vec::new();
    while let Some(line) = lines.next() {
        for b in line.bytes() {
            let d = match b {
                b'>' => 0,
                b'^' => 1,
                b'<' => 2,
                b'v' => 3,
                _ => panic!(),
            };
            moves.push(d);
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (1497888 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (1522420 as usize).solution());
    }
}
