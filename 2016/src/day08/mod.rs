use itertools::Itertools;

use crate::{IntoSolution, Solution};

pub const TITLE: &str = "Two-Factor Authenticationy";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    const W: usize = 50;
    const H: usize = 6;

    let mut screen = vec![vec![0; H]; W];
    let ops = parse();
    
    for op in ops {
        match op {
            Op::Rect(w, h) => {
                for x in 0..w {
                    for y in 0..h {
                        screen[x as usize][y as usize] = 1;
                    }
                }
            },
            Op::RotCol(c, a) => {
                screen[c as usize].rotate_right(a as usize);
            },
            Op::RotRow(r, a) => {
                for _ in 0..a  {
                    let tmp = screen[W - 1][r as usize];
                    for i in (0..W).rev() {
                        screen[i % W][r as usize] = screen[(i - 1) % W][r as usize];
                    }
                    screen[0][r as usize] = tmp;
                }
            },
        }
    }

    print_screen(&screen, W, H);

    let s: u32 = screen.iter().map(|x| x.iter().sum::<u32>()).sum();
    return s.solution();
}

pub fn part2() -> Option<Solution> {
    None
}

fn print_screen(screen: &Vec<Vec<u32>>, w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            let v = screen[x][y];
            print!("{}", if v != 0 { '#' } else { ' ' });
        }
        println!();
    }
}

fn parse() -> Vec::<Op> {
    let mut ops = Vec::<Op>::new();
    for line in INPUT.lines() {
        let tokens = line.split_whitespace().collect_vec();
        match tokens[0] {
            "rect" => {
                let v = tokens[1].split('x').collect_vec();
                ops.push(Op::Rect(v[0].parse().unwrap(), v[1].parse().unwrap()))
            },
            "rotate" => {
                match tokens[1] {
                    "row" => {
                        let row = tokens[2].split('=').skip(1).next().unwrap().parse().unwrap();
                        let a = tokens[4].parse().unwrap();
                        ops.push(Op::RotRow(row, a))
                    },
                    "column" => {
                        let col = tokens[2].split('=').skip(1).next().unwrap().parse().unwrap();
                        let a = tokens[4].parse().unwrap();
                        ops.push(Op::RotCol(col, a))
                    },
                    _ => panic!()
                }
            },
            _ => panic!()
        };
    }

    ops
}

#[derive(Debug)]
enum Op {
    Rect(u32, u32),
    RotCol(u32, u32),
    RotRow(u32, u32),
}