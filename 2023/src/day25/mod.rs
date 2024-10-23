use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::solution::{IntoSolution, Solution};

pub const TITLE: &str = "Snowverload";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (g, v, _) = parse();
    let r = solve(g);
    ((v - r) * r).solution()
}


pub fn part2() -> Option<Solution> {
    None
}

fn parse() -> (Vec<Node>, usize, usize) {
    let mut g = Vec::<Node>::new();
    let mut ids = HashMap::<String, usize>::new();
    let mut i = 0;
    let mut e = 0;
    for line in INPUT.lines() {
        let (id, ns) = line.split_once(':').unwrap();

        if !ids.contains_key(id) {
            ids.insert(id.to_string(), i);
            i += 1;
            g.push(Node::new());
        }

        let id = ids[id];
        for n in ns.split(' ') {
            if n.is_empty() {
                continue;
            }

            if !ids.contains_key(n) {
                ids.insert(n.to_string(), i);
                i += 1;
                g.push(Node::new());
            }

            let n = ids[n];
            g[id].id = id;
            g[id].neighbours.push((n, 1));
            g[n].id = n;
            g[n].neighbours.push((id, 1));
            e += 1;
        }
    }
    (g, i, e)
}

fn solve(g: Vec<Node>) -> usize {
    let mut found = vec![false; g.len()];
    found[0] = true;

    let mut count = 1;
    loop {
        let potential = g
            .iter()
            .filter(|x| !found[x.id])
            .map(|x| (x.id, x.neighbours.iter().filter(|c| found[c.0]).count()))
            .filter(|x| x.1 != 0)
            .collect_vec();
        
        let (id, w) = potential.iter().max_by_key(|x| x.1).unwrap();

        if potential.len() == 3 && *w == 1 {
            break;
        }

        found[*id] = true;
        count += 1;
    }

    count
}

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    neighbours: Vec<(usize, usize)>,
}

impl Node {
    fn new() -> Self {
        Self {
            id: 0,
            neighbours: Vec::new(),
        }
    }
}
