use std::{self, collections::{HashSet, HashMap}};
use crate::utils;

pub fn d3_1() {
    let mut score = 0;
    let score_map = score_map();

    let mut reader = utils::Reader::load_input("src/day3/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l = line.unwrap().trim();
        let halves = l.split_at(l.len() / 2);
        let first: HashSet<char> = HashSet::from_iter(halves.0.chars());
        let last: HashSet<char> = HashSet::from_iter(halves.1.chars());
        let mut k = first.intersection(&last);
        score += score_map[k.next().unwrap()];
    }

    println!("{}", score);
}

pub fn d3_2() {
    let score_map = score_map();

    let reader = utils::Reader::load_input("src/day3/input.txt").unwrap();
    let score: u32 = reader
        .map(|x| x.unwrap())
        .collect::<Vec::<_>>()
        .chunks(3)
        .map(|x| x
            .iter()
            .map(|a| a.trim().chars().collect::<HashSet<char>>())
            .reduce(|a, b| a.intersection(&b).copied().collect())
        )
        .map(|x| score_map[x.unwrap().iter().next().unwrap()] as u32)
        .sum();
    
    println!("{}", score);
}

fn score_map() -> HashMap<char, usize> {
    let alphabet: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let values = alphabet
        .chars()
        .enumerate()
        .map(|(i, x)| (x, i + 1))
        .collect::<HashMap<_, _>>();
    values
}