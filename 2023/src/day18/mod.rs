use crate::utils::{solution::{Solution, IntoSolution}, vec2d::ADJ_FOUR, Parsable};

pub const TITLE: &str = "Lavaduct Lagoon";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let insts = parse();
    solve(&insts).solution()
}

pub fn part2() -> Option<Solution> {
    let insts = parse2();
    solve(&insts).solution()
}

fn parse() -> Vec<Instruction> {
    let mut insts = Vec::new();
    for line in INPUT.lines() {
        let mut bytes = line.bytes();
        let dir = match bytes.next().unwrap() {
            b'R' => 0,
            b'U' => 1,
            b'L' => 2,
            b'D' => 3,
            _ => panic!(),
        };

        let len: usize = bytes.next_number().unwrap();
        insts.push(Instruction { dir, len })
    }

    insts
}

fn parse2() -> Vec<Instruction> {
    let mut insts = Vec::new();
    for line in INPUT.lines() {
        let mut bytes = line.bytes();
        
        bytes.next();
        let _: usize = bytes.next_number().unwrap();
        bytes.nth(1);

        let mut len = 0;
        for i in (0..5).rev() {
            let ch = bytes.next().unwrap() as char;
            len += ch.to_digit(16).unwrap() * 16u32.pow(i);
        }

        let dir = match bytes.next().unwrap() {
            b'0' => 0,
            b'1' => 3,
            b'2' => 2,
            b'3' => 1,
            _ => panic!(),
        };
        
        insts.push(Instruction { dir, len: len as usize })
    }

    insts
}

fn solve(instructions: &Vec<Instruction>) -> usize {
    let mut ps = Vec::<Position>::new();
    let mut p = Position { x: 0, y: 0 };
    ps.push(p.clone());
    let mut l = 0;
    for inst in instructions {
        p.x += ADJ_FOUR[inst.dir].0 * inst.len as isize;
        p.y += ADJ_FOUR[inst.dir].1 * inst.len as isize;
        l += inst.len;
        ps.push(p.clone());
    }

    // Pick's
    area(&ps) + l / 2 + 1
}

// Shoelace
fn area(verts: &Vec<Position>) -> usize {
    (0..verts.len())
        .map(|i| det(&verts[i], &verts[(i + 1) % verts.len()]))
        .sum::<isize>() as usize
        / 2
}

fn det(a: &Position, b: &Position) -> isize {
    a.x * b.y - b.x * a.y
}

#[derive(Debug, Clone)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Instruction {
    dir: usize,
    len: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 53844),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 42708339569950),
            _ => panic!(),
        }
    }
}
