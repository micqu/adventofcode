use std::collections::HashMap;

use crate::{map, IntoSolution, Solution};

pub const TITLE: &str = "Signals and Noise";
const INPUT: &'static str = include_str!("input.txt");

const LENGTH: usize = 8;

pub fn part1() -> Option<Solution> {
    let mut password = String::with_capacity(LENGTH);
    for map in parse() {
        let m = map.iter().max_by_key(|x| x.1).unwrap();
        password.push(*m.0);
    }
    
    password.solution()
}

pub fn part2() -> Option<Solution> {
    let mut password = String::with_capacity(LENGTH);
    for map in parse() {
        let m = map.iter().min_by_key(|x| x.1).unwrap();
        password.push(*m.0);
    }
    
    password.solution()
}

fn parse() -> Vec::<HashMap<char, u32>> {
    let mut counts = vec![map!(char, u32); LENGTH];
    for line in INPUT.lines() {
        let chs = line.trim().chars();
        for (i, ch) in chs.enumerate() {
            *counts[i].entry(ch).or_insert(0) += 1;
        }
    }
    counts
}