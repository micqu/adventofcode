use itertools::Itertools;

use crate::utils;

pub fn d24() {
    let numbers = parse("src/day24/input.txt");
    let sum: u64 = numbers.iter().sum();

    let result = solve(&numbers, sum / 3);
    println!("{}", result.unwrap());

    let result = solve(&numbers, sum / 4);
    println!("{}", result.unwrap());
}

fn solve(v: &Vec<u64>, target: u64) -> Option<u64> {
    for i in 2..=v.len() {
        let mut groups_qe = Vec::<u64>::new();

        for pair in v.iter().copied().combinations(i) {
            if pair.iter().sum::<u64>() == target {
                groups_qe.push(pair.iter().product());
            }
        }
        
        groups_qe.sort_unstable();
        
        if let Some(s) = groups_qe.first() {
            return Some(*s);
        }
    }
    
    None
}

fn parse(file: &str) -> Vec<u64> {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    let mut numbers = Vec::<u64>::new();

    for line in buffer.lines() {
        let number = line.trim().parse().unwrap();
        numbers.push(number);
    }
    
    numbers
}