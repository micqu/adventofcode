use std::collections::{HashMap, HashSet};

use crate::utils;

pub fn d17() {
    d17_1();
    d17_2();
}

pub fn d17_1() {
    let mut containers: Vec<Container> = Vec::new();
    parse("src/day17/input.txt", &mut containers);
    let r = solve(
        150,
        &State {
            curr: 0,
            remaining: containers,
            n_containers: 0,
        },
        &mut HashSet::<State>::new(),
    );
    println!("{}", r);
}

pub fn d17_2() {
    let mut containers: Vec<Container> = Vec::new();
    parse("src/day17/input.txt", &mut containers);
    let mut n_count = HashMap::<u32, u32>::new();
    solve2(
        150,
        &State {
            curr: 0,
            remaining: containers,
            n_containers: 0,
        },
        &mut HashSet::<State>::new(),
        &mut n_count,
    );
    let r = *n_count.iter().min_by_key(|(k, _)| **k).unwrap().1;
    println!("{}", r);
}

fn solve(eggnog: u32, state: &State, visited: &mut HashSet<State>) -> u32 {
    if !visited.insert(state.clone()) {
        return 0;
    }

    if state.curr == eggnog {
        return 1;
    }

    let mut c = 0;
    for r in state.remaining.iter() {
        let ns = State {
            curr: state.curr + r.vol,
            remaining: state
                .remaining
                .iter()
                .filter(|&x| x.id != r.id)
                .map(|x| x.clone())
                .collect(),
            n_containers: state.n_containers + 1,
        };
        c += solve(eggnog, &ns, visited);
    }
    c
}

fn solve2(
    eggnog: u32,
    state: &State,
    visited: &mut HashSet<State>,
    n_count: &mut HashMap<u32, u32>,
) -> u32 {
    if !visited.insert(state.clone()) {
        return 0;
    }

    if state.curr == eggnog {
        *n_count.entry(state.n_containers).or_insert(0) += 1;
        return 1;
    }

    let mut c = 0;
    for r in state.remaining.iter() {
        let ns = State {
            curr: state.curr + r.vol,
            remaining: state
                .remaining
                .iter()
                .filter(|&x| x.id != r.id)
                .map(|x| x.clone())
                .collect(),
            n_containers: state.n_containers + 1,
        };
        c += solve2(eggnog, &ns, visited, n_count);
    }
    c
}

fn parse(file: &str, containers: &mut Vec<Container>) {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    buffer.lines().enumerate().for_each(|(i, line)| {
        let c = Container {
            id: i as u32,
            vol: line.trim().parse().unwrap(),
        };
        containers.push(c);
    });
}

#[derive(Debug, Hash, Clone)]
struct State {
    curr: u32,
    remaining: Vec<Container>,
    n_containers: u32,
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        let k = HashSet::<Container>::from_iter(self.remaining.iter().cloned())
            .difference(&HashSet::from_iter(other.remaining.iter().cloned()))
            .count();

        self.curr == other.curr && k == 0
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Container {
    id: u32,
    vol: u32,
}
