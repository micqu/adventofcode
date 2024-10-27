use std::collections::HashSet;
use itertools::Itertools;

use crate::{IntoSolution, Solution};

pub const TITLE: &str = "Internet Protocol Version 7";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut count = 0;
    for line in INPUT.lines() {
        let chs = line.trim().chars().collect_vec();

        let mut abba_in_hypernet = false;
        let mut abba = false;
        let mut in_window = false;
        for w in chs.windows(4) {
            if w[0] == '[' {
                in_window = true;
            }
            if w[0] == ']' {
                in_window = false;
            }

            if w[0] != w[1] && w[1] == w[2] && w[0] == w[3] {
                if in_window {
                    abba_in_hypernet = true;
                } else {
                    abba = true;
                }
            }
        }

        if abba && !abba_in_hypernet {
            count += 1;
        }
    };

    count.solution()
}

pub fn part2() -> Option<Solution> {
    let mut count = 0;
    for line in INPUT.lines() {
        let chs = line.trim().chars().collect_vec();

        let mut abas = HashSet::<String>::new();
        let mut babs = HashSet::<String>::new();
        let mut in_window = false;
        for w in chs.windows(3) {
            if w[0] == '[' {
                in_window = true;
            }
            if w[0] == ']' {
                in_window = false;
            }

            if w[0] != w[1] && w[0] == w[2] {
                if in_window {
                    babs.insert([w[1], w[0], w[1]].iter().collect());
                } else {
                    abas.insert(w.iter().collect());
                }
            }
        }

        if abas.intersection(&babs).count() > 0 {
            count += 1;
        }
    };

    count.solution()
}