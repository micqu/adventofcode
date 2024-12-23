use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::format,
};

use itertools::Itertools;
use rayon::{slice::ParallelSliceMut, vec};

use crate::utils::solution::{IntoSolution, Solution};

pub const TITLE: &str = "LAN Party";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (neighbours, adj) = parse();

    let mut seen = vec![false; adj.len()];
    let mut s: usize = 0;
    for t in 19 * 26..20 * 26 {
        if let Some(n) = neighbours.get(&t) {
            seen[t] = true;
            for (i, &a) in n.iter().enumerate() {
                for &b in n.iter().skip(i) {
                    if !seen[a] && !seen[b] && adj[a][b] {
                        s += 1;
                    }
                }
            }
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let (neighbours, adj) = parse();

    let mut max_clique = Vec::new();
    let mut clique = Vec::<usize>::new();
    let mut seen = vec![false; 26 * 26];
    for (node, ns) in neighbours {
        if seen[node] {
            continue;
        }

        seen[node] = true;
        clique.push(node);

        for n in ns {
            if clique.iter().all(|x| adj[*x][n]) {
                clique.push(n);
                seen[n] = true;
            }
        }

        if clique.len() > max_clique.len() {
            max_clique = clique.clone();
        }

        clique.clear();
    }

    let mut s = Vec::new();
    for id in max_clique {
        s.push(format!(
            "{}{}",
            (((id / 26) as u8) + b'a') as char,
            (((id % 26) as u8) + b'a') as char,
        ));
    }

    s.iter().sorted_unstable().join(",").solution()
}

fn parse() -> (HashMap<usize, Vec<usize>>, Vec<Vec<bool>>) {
    let mut adj = vec![vec![false; 26 * 26]; 26 * 26];
    let mut neighbours = HashMap::new();

    for line in INPUT.lines() {
        let mut bytes = line.bytes();
        let a1 = (bytes.next().unwrap() - b'a') as usize;
        let b1 = (bytes.next().unwrap() - b'a') as usize;
        let n1 = a1 * 26 + b1;

        bytes.next();

        let a2 = (bytes.next().unwrap() - b'a') as usize;
        let b2 = (bytes.next().unwrap() - b'a') as usize;
        let n2 = a2 * 26 + b2;

        adj[n1][n2] = true;
        adj[n2][n1] = true;

        neighbours.entry(n1).or_insert(Vec::new()).push(n2);
        neighbours.entry(n2).or_insert(Vec::new()).push(n1);
    }

    (neighbours, adj)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (1175 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(),
            "bw,dr,du,ha,mm,ov,pj,qh,tz,uv,vq,wq,xw".solution()
        );
    }
}
