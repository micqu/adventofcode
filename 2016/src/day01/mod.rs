use std::collections::HashSet;

use crate::{IntoSolution, Solution};

pub const TITLE: &str = "No Time for a Taxicab";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let moves = parse();

    const DX: [i32; 4] = [1, 0, -1, 0];
    const DY: [i32; 4] = [0, 1, 0, -1];

    let mut dir: i32 = 1;
    let mut x = 0;
    let mut y = 0;

    for m in moves {
        match m {
            Move::Left(d) => {
                dir = (dir + 1) % 4;
                x += DX[dir as usize] * d as i32;
                y += DY[dir as usize] * d as i32;
            }
            Move::Right(d) => {
                dir = (dir - 1).rem_euclid(4);
                x += DX[dir as usize] * d as i32;
                y += DY[dir as usize] * d as i32;
            }
        }
    }

    (x.abs() + y.abs()).solution()
}

pub fn part2() -> Option<Solution> {
    let moves = parse();

    let mut map = HashSet::<(i32, i32)>::new();
    const DX: [i32; 4] = [1, 0, -1, 0];
    const DY: [i32; 4] = [0, 1, 0, -1];

    let mut dir: i32 = 1;
    let mut x = 0;
    let mut y = 0;

    if !map.insert((x, y)) {
        return (x.abs() + y.abs()).solution();
    }

    for m in moves {
        match m {
            Move::Left(d) => {
                dir = (dir + 1) % 4;
                for _ in 0..d {
                    x += DX[dir as usize];
                    y += DY[dir as usize];
                    if !map.insert((x, y)) {
                        return (x.abs() + y.abs()).solution();
                    }
                }
            }
            Move::Right(d) => {
                dir = (dir - 1).rem_euclid(4);
                for _ in 0..d {
                    x += DX[dir as usize];
                    y += DY[dir as usize];
                    if !map.insert((x, y)) {
                        return (x.abs() + y.abs()).solution();
                    }
                }
            }
        }
    }

    None
}

fn parse() -> Vec<Move> {
    let mut moves = Vec::<Move>::new();
    for line in INPUT.lines() {
        let split: Vec<&str> = line.trim().split(", ").collect();

        for m in split {
            let mut ch = m.chars();
            let r = ch.next().unwrap();
            let d = ch.as_str().parse::<u32>().unwrap();
            match r {
                'L' => moves.push(Move::Left(d)),
                'R' => moves.push(Move::Right(d)),
                _ => panic!(),
            }
        }
    }

    moves
}

#[derive(Debug)]
enum Move {
    Left(u32),
    Right(u32),
}
