use std::str::{Bytes, Lines};

use num::integer::lcm;

use crate::utils::solution::{Solution, IntoSolution};

pub const TITLE: &str = "Haunted Wasteland";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut lines = INPUT.lines();
    let (insts, map) = parse(&mut lines);

    let g = compose_node_id(b'Z', b'Z', b'Z');
    let mut c = compose_node_id(b'A', b'A', b'A');
    let mut i: u32 = 0;
    for inst in insts.iter().cycle() {
        let next = decompose_node(map[c as usize]);

        if next.0 == g {
            break;
        }

        c = match inst {
            b'L' => next.1,
            b'R' => next.2,
            _ => panic!(),
        };

        i += 1;
    }

    i.solution()
}

pub fn part2() -> Option<Solution> {
    let mut lines = INPUT.lines();
    let (insts, map) = parse(&mut lines);

    let mut starts = vec![0];
    for node in map.iter() {
        if *node == 0 {
            continue;
        }

        let n = decompose_node(*node);
        if decompose_node_id(n.0).2 == 0 {
            starts.push(*node);
        }
    }

    starts.iter().map(|start| {
        let c = decompose_node(*start).0;
        insts.iter().cycle().scan(c, |c, step| {
            let next = decompose_node(map[*c as usize]);
            
            *c = match step {
                b'L' => next.1,
                b'R' => next.2,
                _ => panic!(),
            };
            
            Some((*c & 0b11111) as u8 == 25)
        }).position(|c| c).unwrap() + 1
    }).fold(1, lcm).solution()
}

fn parse(lines: &mut Lines) -> (Vec<u8>, Vec<u64>) {
    let insts: Vec<u8> = lines.next().unwrap().bytes().collect();
    let mut map = vec![0; 26426];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let mut line = line.bytes();
        let id: u16 = read_node(&mut line);
        line.nth(3);
        let left = read_node(&mut line);
        line.nth(1);
        let right = read_node(&mut line);

        map[id as usize] = compose_node(id, left, right);
    }
    (insts, map)
}

fn read_node(input: &mut Bytes) -> u16 {
    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let c = input.next().unwrap();
    ((((a - b'A') as u16) << 10) | ((b - b'A') as u16) << 5) | ((c - b'A') as u16)
}

fn compose_node(id: u16, left: u16, right: u16) -> u64 {
    ((id as u64) << 32) | ((left as u64) << 16) | right as u64
}

fn decompose_node(a: u64) -> (u16, u16, u16) {
    let right = a & 0b1111111111111111;
    let left = (a & (0b1111111111111111 << 16)) >> 16;
    let id = (a & (0b1111111111111111 << 32)) >> 32;
    (id as u16, left as u16, right as u16)
}

fn compose_node_id(a: u8, b: u8, c: u8) -> u16 {
    ((((a - b'A') as u16) << 10) | ((b - b'A') as u16) << 5) | ((c - b'A') as u16)
}

fn decompose_node_id(node: u16) -> (u8, u8, u8) {
    let a = node >> 10;
    let b = (node & (0b11111 << 5)) >> 5;
    let c = node & 0b11111;
    (a as u8, b as u8, c as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 17873),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 15746133679061),
            _ => panic!(),
        }
    }
}