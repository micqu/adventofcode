use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    time::Instant,
};

use itertools::{Itertools, TupleCombinations};

use crate::utils::{
    Parsable,
    solution::{IntoSolution, Solution},
    vec3::Vec3,
};

pub const TITLE: &str = "Playground";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let boxes = parse();

    let mut edges = boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((i, a), (j, b))| (a.l2(b) as usize, i, j))
        .collect_vec();

    let (lesser, _, _) = edges.select_nth_unstable_by_key(1000, |a| a.0);
    let mut circuits = vec![1; boxes.len()];
    let mut loc = (0..boxes.len()).collect_vec();

    for (_, a, b) in lesser {
        if loc[*a] != loc[*b] {
            let (n, m) = (loc[*a].min(loc[*b]), loc[*a].max(loc[*b]));
            circuits[n] += circuits[m];
            circuits[m] = 0;

            for l in &mut loc {
                if *l == m {
                    *l = n;
                }
            }
        }
    }

    circuits
        .iter()
        .sorted_unstable()
        .product::<usize>()
        .solution()
}

pub fn part2() -> Option<Solution> {
    let boxes = parse();

    let mut edges = BinaryHeap::new();
    for ((i, a), (j, b)) in boxes.iter().enumerate().tuple_combinations() {
        edges.push(Reverse((a.l2(b) as usize, (i, j))));
    }

    let mut circuits = vec![1; boxes.len()];
    let mut loc = (0..boxes.len()).collect_vec();

    let mut count = 0;
    while let Some(Reverse((_, (a, b)))) = edges.pop() {
        if loc[a] != loc[b] {
            let (n, m) = (loc[a].min(loc[b]), loc[a].max(loc[b]));
            circuits[n] += circuits[m];
            circuits[m] = 0;

            count += 1;
            if count == boxes.len() - 1 {
                return (boxes[a].x * boxes[b].x).solution();
            }

            for l in &mut loc {
                if *l == m {
                    *l = n;
                }
            }
        }
    }

    None
}

fn parse() -> Vec<Vec3> {
    INPUT
        .lines()
        .map(|line| Vec3::parse(&mut line.bytes()))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (115885 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (274150525 as f64).solution());
    }
}
