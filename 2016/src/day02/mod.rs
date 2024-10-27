use std::ops::Add;

use crate::{IntoSolution, Solution};

pub const TITLE: &str = "Bathroom Security";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let moves = parse();

    const KEYPAD: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    let mut x = 1;
    let mut y = 1;

    let mut result = 0;
    for line_moves in moves {
        for m in line_moves {
            x = x.add(m.dx).clamp(0, 2);
            y = y.add(m.dy).clamp(0, 2);
        }

        result = result * 10 + KEYPAD[y as usize][x as usize];
    }
    
    return result.solution();
}

pub fn part2() -> Option<Solution> {
    let moves = parse();
    
    const KEYPAD: [[char; 5]; 5] = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];

    let mut x = 0;
    let mut y = 2;

    let mut result = Vec::<char>::new();
    for line_moves in moves {
        for m in line_moves {
            let nx = x.add(m.dx).clamp(0, 4);
            let ny = y.add(m.dy).clamp(0, 4);
            if KEYPAD[ny as usize][nx as usize] != ' ' {
                x = nx;
                y = ny;
            }
        }
        
        result.push(KEYPAD[y as usize][x as usize]);
    }
    
    result.iter().collect::<String>().solution()
}

fn parse() -> Vec<Vec<Move>> {
    let mut moves = Vec::<Vec<Move>>::new();
    for line in INPUT.lines() {
        let mut line_moves = Vec::<Move>::new();
        for ch in line.chars() {
            match ch  {
                'L' => line_moves.push(Move { dx: -1, dy: 0 }),
                'R' => line_moves.push(Move { dx: 1, dy: 0 }),
                'U' => line_moves.push(Move { dx: 0, dy: -1 }),
                'D' => line_moves.push(Move { dx: 0, dy: 1 }),
                _ => { }
            }
        }
        moves.push(line_moves);
    }
    
    moves
}

#[derive(Debug)]
struct Move {
    dx: i32,
    dy: i32
}