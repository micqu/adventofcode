use core::panic;

use itertools::Itertools;

use crate::utils::solution::{Solution, IntoSolution};

const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (m, start) = parse();
    let s = trace(start, &m).len() / 2;

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let (m, start) = parse();
    let tiles = trace(start, &m);
    let s = area(&tiles) - tiles.len() / 2 + 1; // Pick's

    s.solution()
}

fn parse() -> (Vec<Vec<u8>>, Position) {
    let mut s = Position { x: 0, y: 0};
    let m = INPUT.lines().enumerate().map(|(i, line)| {
        let line = line.bytes().collect_vec();
        if let Some(p) = line.iter().position(|c| *c == b'S') {
            s.x = p as isize;
            s.y = i as isize;
        }
        line
    }).collect_vec();

    (m, s)
}

fn get_orientation(s: Position, map: &Vec<Vec<u8>>) -> Direction {
    let (x, y) = (s.x as usize, s.y as usize);
    let up = [b'7', b'F', b'|'].contains(&map[y - 1][x]);
    let down = [b'J', b'L', b'|'].contains(&map[y + 1][x]);
    let left = [b'F', b'L', b'-'].contains(&map[y][x - 1]);
    let right = [b'J', b'7', b'-'].contains(&map[y][x + 1]);
    
    match (up, down, left, right) {
        (true, _, _, _) => Direction::Up,
        (_, true, _, _) => Direction::Down,
        (_, _, true, _) => Direction::Left,
        (_, _, _, true) => Direction::Right,
        _ => panic!(),
    }
}

fn trace(s: Position, map: &Vec<Vec<u8>>) -> Vec<Position> {
    let (mut x, mut y) = (s.x as usize, s.y as usize);
    let mut visited = Vec::<Position>::new();
    let mut dir = get_orientation(s, &map);
    loop {
        match dir {
            Direction::Up => {
                dir = match map[y - 1][x] {
                    b'7' => Direction::Left,
                    b'F' => Direction::Right,
                    _ => dir,
                };
                y -= 1;
            },
            Direction::Down => {
                dir = match map[y + 1][x] {
                    b'J' => Direction::Left,
                    b'L' => Direction::Right,
                    _ => dir,
                };
                y += 1;
            },
            Direction::Left => {
                dir = match map[y][x - 1] {
                    b'L' => Direction::Up,
                    b'F' => Direction::Down,
                    _ => dir,
                };
                x -= 1;
            },
            Direction::Right => {
                dir = match map[y][x + 1] {
                    b'J' => Direction::Up,
                    b'7' => Direction::Down,
                    _ => dir,
                };
                x += 1;
            }
        }

        visited.push(Position {
            x: x as isize,
            y: y as isize,
        });

        if map[y][x] == b'S' {
            break;
        }
    }

    visited
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Shoelace
fn area(tiles: &Vec<Position>) -> usize {
    let mut a = 0;
    for i in 0..tiles.len() {
        a += det(&tiles[i], &tiles[(i + 1) % tiles.len()]);
    }
    a.abs() as usize / 2
}

fn det(a: &Position, b: &Position) -> isize {
    a.x * b.y - b.x * a.y
}