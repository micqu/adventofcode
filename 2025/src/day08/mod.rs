use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    time::Instant,
};

use itertools::{Itertools, TupleCombinations};

use crate::utils::{
    Parsable,
    points::point3d::Point3d,
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
        .map(|((i, a), (j, b))| (l2(a, b) as usize, i, j))
        .collect_vec();

    let (lesser, _, _) = edges.select_nth_unstable_by_key(1000, |a| a.0);
    let mut circuits = vec![1; boxes.len()];
    let mut loc = (0..boxes.len()).collect_vec();

    for (_, a, b) in lesser {
        let a = find(*a, &mut loc);
        let b = find(*b, &mut loc);

        if loc[a] != loc[b] {
            let (n, m) = (loc[a].min(loc[b]), loc[a].max(loc[b]));
            circuits[n] += circuits[m];
            circuits[m] = 0;
            loc[a] = n;
            loc[b] = n;
        }
    }

    let (lesser, _, _) = circuits.select_nth_unstable_by(3, |a, b| b.cmp(&a));
    lesser.iter().product::<usize>().solution()
}

pub fn part2() -> Option<Solution> {
    let boxes = parse();

    let mut edges = BinaryHeap::new();
    for ((i, a), (j, b)) in boxes.iter().enumerate().tuple_combinations() {
        edges.push(Reverse((l2(a, b) as usize, (i, j))));
    }

    let mut circuits = vec![1; boxes.len()];
    let mut loc = (0..boxes.len()).collect_vec();

    let mut count = 0;
    while let Some(Reverse((_, pair))) = edges.pop() {
        let a = find(pair.0, &mut loc);
        let b = find(pair.1, &mut loc);

        if loc[a] != loc[b] {
            let (n, m) = (loc[a].min(loc[b]), loc[a].max(loc[b]));
            circuits[n] += circuits[m];
            circuits[m] = 0;

            count += 1;
            if count == boxes.len() - 1 {
                return (boxes[pair.0].x * boxes[pair.1].x).solution();
            }

            loc[a] = n;
            loc[b] = n;
        }
    }

    None
}

fn parse() -> Vec<Point3d<isize>> {
    INPUT
        .lines()
        .map(|line| Point3d::<isize>::parse(&mut line.bytes()))
        .collect_vec()
}

fn l2(a: &Point3d<isize>, b: &Point3d<isize>) -> isize {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

fn find(i: usize, loc: &mut Vec<usize>) -> usize {
    let mut r = i;
    while loc[r] != r {
        r = loc[r];
    }

    let mut c = i;
    while c != r {
        let n = loc[c];
        loc[c] = r;
        c = n;
    }
    r
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
