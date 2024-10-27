use std::{str::FromStr, collections::HashMap};

use itertools::Itertools;

use crate::{IntoSolution, Solution};

pub const TITLE: &str = "Security Through Obscurity";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut sum = 0;
    for room in parse() {
        if room.is_real() {
            sum += room.sector_id;
        }
    }

    sum.solution()
}

pub fn part2() -> Option<Solution> {
    for room in parse() {
        if room.get_decrypted_name().contains("north") {
            return room.sector_id.solution();
        }
    }

    None
}

fn parse() -> Vec::<Room> {
    let mut rooms = Vec::<Room>::new();

    for line in INPUT.lines() {
        let parts = line
            .split_terminator(&['-', '[', ']', '\n'])
            .filter(|x| x.len() > 0)
            .collect::<Vec<&str>>();

        let names = parts[0..parts.len() - 2].iter()
            .map(|x| String::from_str(x).unwrap())
            .collect_vec();

        let room = Room {
            names: names,
            sector_id: parts[parts.len() - 2].parse().unwrap(),
            checksum: String::from_str(parts[parts.len() - 1]).unwrap(),
        };

        rooms.push(room);
    }

    rooms
}

#[derive(Debug)]
struct Room {
    names: Vec<String>,
    sector_id: u32,
    checksum: String
}

impl Room {
    fn is_real(&self) -> bool {
        let mut freq_map = HashMap::<char, u32>::new();

        for name in &self.names {
            for ch in name.chars() {
                *freq_map.entry(ch).or_insert(0) += 1;
            }
        }

        let mut ordered = freq_map.iter().collect_vec();
        ordered.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        let o: String = ordered.iter().take(5).map(|x| x.0).collect();
        return o == self.checksum;
    }

    fn get_decrypted_name(&self) -> String {
        self.names
            .iter()
            .map(|name| {
                name.chars()
                .map(|x| ((x as u32) + self.sector_id - 97) % 26 + 97)
                .map(|x| char::from_u32(x).unwrap())
                .collect::<String>()
            })
            .join(" ")
    }
}