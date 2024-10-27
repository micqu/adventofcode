use core::panic;
use std::collections::HashMap;

use itertools::Itertools;

use crate::{map, IntoSolution, Solution};

pub const TITLE: &str = "Balance Bots";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (mut bots, mut outputs) = parse();

    let mut tracker = map!(usize, usize);
    let mut done = false;
    while !done {
        done = true;
        for bot_id in 0..bots.len() {
            if let Some(transactions) = send(bot_id, &bots, &outputs) {
                for t in transactions {
                    if let Entity::Bot(from) = t.from {
                        let v = *tracker.entry(from).or_insert(t.value);
                        if v.min(t.value) == 17 && v.max(t.value) == 61 {
                            return from.solution();
                        }
                    }

                    match t.from {
                        Entity::Bot(from) => match t.to {
                            Entity::Bot(to) => {
                                bots.get_mut(&to).unwrap().microchips.push(t.value);
                                bots.get_mut(&from)
                                    .unwrap()
                                    .microchips
                                    .retain(|x| *x != t.value);
                            }
                            Entity::Output(to) => {
                                outputs.get_mut(&to).unwrap().microchips.push(t.value);
                                bots.get_mut(&from)
                                    .unwrap()
                                    .microchips
                                    .retain(|x| *x != t.value);
                            }
                        },
                        _ => panic!(),
                    }
                }

                done = false;
            }
        }
    }

    None
}

pub fn part2() -> Option<Solution> {
    let (mut bots, mut outputs) = parse();
    let mut done = false;
    while !done {
        done = true;
        for bot_id in 0..bots.len() {
            if let Some(transactions) = send(bot_id, &bots, &outputs) {
                for t in transactions {
                    match t.from {
                        Entity::Bot(from) => match t.to {
                            Entity::Bot(to) => {
                                bots.get_mut(&to).unwrap().microchips.push(t.value);
                                bots.get_mut(&from)
                                    .unwrap()
                                    .microchips
                                    .retain(|x| *x != t.value);
                            }
                            Entity::Output(to) => {
                                outputs.get_mut(&to).unwrap().microchips.push(t.value);
                                bots.get_mut(&from)
                                    .unwrap()
                                    .microchips
                                    .retain(|x| *x != t.value);
                            }
                        },
                        _ => panic!(),
                    }
                }

                done = false;
            }
        }
    }

    outputs
        .iter()
        .sorted_by(|a, b| a.0.cmp(b.0))
        .take(3)
        .map(|x| x.1.microchips.iter().next().unwrap())
        .product::<usize>()
        .solution()
}

fn parse() -> (HashMap<usize, Bot>, HashMap<usize, Output>) {
    let mut bots = HashMap::<usize, Bot>::new();
    let mut outputs = HashMap::<usize, Output>::new();
    for line in INPUT.lines() {
        let split: Vec<&str> = line.trim().split_whitespace().collect();
        match split[0] {
            "value" => {
                let id = split[5].parse::<usize>().unwrap();
                let bot = bots.entry(id).or_insert(Bot::default());
                bot.id = id;

                let val = split[1].parse::<usize>().unwrap();
                bot.microchips.push(val);
            }
            "bot" => {
                let id = split[1].parse::<usize>().unwrap();
                let bot = bots.entry(id).or_insert(Bot::default());
                bot.id = id;

                let low_id = split[6].parse::<usize>().unwrap();
                match split[5] {
                    "bot" => {
                        bot.low = Some(Entity::Bot(low_id));
                    }
                    "output" => {
                        bot.low = Some(Entity::Output(low_id));
                        outputs.entry(low_id).or_insert(Output::default()).id = low_id;
                    }
                    _ => panic!(),
                }

                let high_id = split[11].parse::<usize>().unwrap();
                match split[10] {
                    "bot" => {
                        bot.high = Some(Entity::Bot(high_id));
                    }
                    "output" => {
                        bot.high = Some(Entity::Output(high_id));
                        outputs.entry(high_id).or_insert(Output::default()).id = high_id;
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
    (bots, outputs)
}

#[derive(Debug)]
struct Bot {
    id: usize,
    microchips: Vec<usize>,
    low: Option<Entity>,
    high: Option<Entity>,
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            id: Default::default(),
            microchips: Default::default(),
            low: Default::default(),
            high: Default::default(),
        }
    }
}

fn send(
    bot_id: usize,
    bots: &HashMap<usize, Bot>,
    outputs: &HashMap<usize, Output>,
) -> Option<Vec<Transaction>> {
    let from = &bots[&bot_id];

    if from.microchips.len() < 2 {
        return None;
    }

    let min = from.microchips.iter().min();
    let max = from.microchips.iter().max();
    if let Some(&min) = min {
        if let Some(&max) = max {
            let mut transactions = Vec::<Transaction>::new();
            if let Some(low) = from.low {
                match low {
                    Entity::Bot(id) => {
                        if let Some(to) = bots.get(&id) {
                            transactions.push(Transaction::to_bot(from, to, min));
                        }
                    }
                    Entity::Output(id) => {
                        if let Some(to) = outputs.get(&id) {
                            transactions.push(Transaction::to_output(from, to, min));
                        }
                    }
                }
            }

            if let Some(high) = from.high {
                match high {
                    Entity::Bot(id) => {
                        if let Some(to) = bots.get(&id) {
                            transactions.push(Transaction::to_bot(from, to, max));
                        }
                    }
                    Entity::Output(id) => {
                        if let Some(to) = outputs.get(&id) {
                            transactions.push(Transaction::to_output(from, to, max));
                        }
                    }
                }
            }

            return Some(transactions);
        }
    }

    None
}

struct Transaction {
    from: Entity,
    to: Entity,
    value: usize,
}

impl Transaction {
    fn to_bot(from: &Bot, to: &Bot, value: usize) -> Self {
        Self {
            from: Entity::Bot(from.id),
            to: Entity::Bot(to.id),
            value,
        }
    }

    fn to_output(from: &Bot, to: &Output, value: usize) -> Self {
        Self {
            from: Entity::Bot(from.id),
            to: Entity::Output(to.id),
            value,
        }
    }
}

#[derive(Debug)]
struct Output {
    id: usize,
    microchips: Vec<usize>,
}

impl Default for Output {
    fn default() -> Self {
        Self {
            id: Default::default(),
            microchips: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Entity {
    Bot(usize),
    Output(usize),
}
