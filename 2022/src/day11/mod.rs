#![allow(dead_code)]

use core::panic;

use crate::utils;

pub fn d11_1() {
    let mut monkeys = parse("src/day11/input.txt");
    
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            monkeys[m].inspections += monkeys[m].items.len() as u64;
            while let Some(item) = monkeys[m].items.pop() {
                let worry: u64 = monkeys[m].calculate_worry(item) / 3;
                let idx = (worry % monkeys[m].test != 0) as usize;
                let t = monkeys[m].targets[idx] as usize;
                monkeys[t].items.insert(0, worry);
            }
        }
    }
    
    let mut inspections = monkeys.iter()
        .map(|x| x.inspections)
        .collect::<Vec<u64>>();
    inspections.sort_unstable();
    println!("{}", inspections[monkeys.len() - 1] * inspections[monkeys.len() - 2]);
}

pub fn d11_2() {
    let mut monkeys = parse("src/day11/input.txt");
    let tsum: u64 = monkeys.iter().map(|x| x.test).product();

    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            monkeys[m].inspections += monkeys[m].items.len() as u64;
            while let Some(item) = monkeys[m].items.pop() {
                let worry: u64 = monkeys[m].calculate_worry(item) % tsum;
                let idx = (worry % monkeys[m].test != 0) as usize;
                let t = monkeys[m].targets[idx] as usize;
                monkeys[t].items.insert(0, worry);
            }
        }
    }
    let mut inspections = monkeys.iter()
        .map(|x| x.inspections)
        .collect::<Vec<u64>>();
    inspections.sort_unstable();
    println!("{}", inspections[monkeys.len() - 1] * inspections[monkeys.len() - 2]);
}

fn parse(file: &str) -> Vec<Monkey> {
    let mut n_monkeys = 0;
    let mut monkeys = Vec::<Monkey>::new();
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l = line.unwrap().trim()
            .split(&[' ', ',', ':'])
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
        
        if l.is_empty() {
            continue;
        }

        match l[0] {
            "Monkey" => {
                monkeys.push(Monkey::default());
                n_monkeys += 1;
            },
            "Starting" => {
                let mut items = l.iter().skip(2).map(|x| (**x).parse::<u64>().unwrap()).rev().collect();
                monkeys[n_monkeys - 1].items.append(&mut items);
            },
            "Operation" => {
                monkeys[n_monkeys - 1].operation = Some(
                    Operation {
                        left: l[3].to_owned(),
                        right: l[5].to_owned(),
                        op: l[4].to_owned()
                    }
                );
            },
            "Test" => {
                monkeys[n_monkeys - 1].test = l[3].parse().unwrap();
            },
            "If" => {
                monkeys[n_monkeys - 1].targets.push(l[5].parse().unwrap());
            },
            _ => panic!()
        }
    }
    monkeys
}

#[derive(Default)]
struct Monkey {
    pub items: Vec::<u64>,
    pub operation: Option<Operation>,
    pub test: u64,
    pub targets: Vec::<u64>,
    pub inspections: u64,
}

impl Monkey {
    pub fn calculate_worry(&self, item: u64) -> u64 {
        let operation = self.operation.as_ref().unwrap();
        let left = match operation.left.as_str() {
            "old" => item,
            a => a.parse().unwrap(),
        };
        let right = match operation.right.as_str() {
            "old" => item,
            a => a.parse().unwrap(),
        };
        match operation.op.as_str() {
            "*" => { left * right },
            "+" => { left + right },
            _ => panic!()
        }
    }
}

struct Operation {
    pub left: String,
    pub right: String,
    pub op: String,
}