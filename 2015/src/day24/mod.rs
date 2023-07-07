use itertools::Itertools;

use crate::utils;

pub fn d24() {
    d24_1();
    d24_2();
}

pub fn d24_1() {
    let numbers = parse("src/day24/input.txt");
    let sum: u64 = numbers.iter().sum();

    let result = solve(numbers, sum / 3);
    println!("{}", result.unwrap());
}

pub fn d24_2() {
    let numbers = parse("src/day24/input.txt");
    let sum: u64 = numbers.iter().sum();

    let result = solve(numbers, sum / 4);
    println!("{}", result.unwrap());
}

fn solve(v: Vec<u64>, target: u64) -> Option<u64> {
    for i in 2..=v.len() {
        let mut groups = Vec::<Group>::new();

        for pair in v.iter().combinations(i) {
            if pair.iter().map(|x| **x).sum::<u64>() == target {
                groups.push(Group {
                    numbers: pair.iter().map(|x| **x).collect_vec()
                })
            }
        }

        groups.sort_by_key(|x| x.calc_entanglement());

        if let Some(s) = groups.first() {
            return Some(s.calc_entanglement());
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
    numbers.reverse();
    numbers
}

#[derive(Debug, PartialEq, Eq)]
struct Group {
    numbers: Vec<u64>
}

impl Group {
    fn calc_entanglement(&self) -> u64 {
        self.numbers.iter().product()
    }
}