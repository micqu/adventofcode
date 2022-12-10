#![allow(dead_code)]

use crate::utils;

pub fn d5_1() {
    let n = 9;
    let mut piles: Vec<Vec<String>> = vec![Vec::new(); n];

    let mut reader = utils::Reader::load_input("src/day5/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l = line.unwrap();

        if l.starts_with("[") || l.starts_with("  ") {
            l.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .enumerate()
                .for_each(|(i, chunk)| {
                    let o = chunk
                        .split_terminator(&['[', ']', '\n', ' '])
                        .collect::<String>();
                    if !o.trim().is_empty() {
                        piles[i].insert(0, o.trim().to_owned().clone());
                    }
                });
        } else {
            if l.trim().is_empty() || l.starts_with(" ") {
                continue;
            }
            let ll: Vec<&str> = l.trim().split_ascii_whitespace().collect();
            let amount: usize = ll[1].parse().unwrap();
            let from: usize = ll[3].parse::<usize>().unwrap();
            let to: usize = ll[5].parse::<usize>().unwrap();
            for _i in 0..amount {
                let c: String = piles[from - 1].pop().unwrap();
                piles[to - 1].push(c.clone());
            }
        }
    }
    
    for mut pile in piles {
        print!("{}", pile.pop().unwrap());
    }
    println!();
}

pub fn d5_2() {
    let n = 9;
    let mut piles: Vec<Vec<String>> = vec![Vec::new(); n];

    let mut reader = utils::Reader::load_input("src/day5/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l = line.unwrap();

        if l.starts_with("[") || l.starts_with("  ") {
            l.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .enumerate()
                .for_each(|(i, chunk)| {
                    let o = chunk
                        .split_terminator(&['[', ']', '\n', ' '])
                        .collect::<String>();
                    if !o.trim().is_empty() {
                        piles[i].insert(0, o.trim().to_owned().clone());
                    }
                });
        } else {
            if l.trim().is_empty() || l.starts_with(" ") {
                continue;
            }
            let ll: Vec<&str> = l.trim().split_ascii_whitespace().collect();
            let amount: usize = ll[1].parse().unwrap();
            let from: usize = ll[3].parse::<usize>().unwrap();
            let to: usize = ll[5].parse::<usize>().unwrap();

            let mut buf = Vec::<String>::new();
            for _i in 0..amount {
                buf.push(piles[from - 1].pop().unwrap());
            }
            buf.reverse();
            piles[to - 1].append(&mut buf);            
        }
    }

    for mut pile in piles {
        print!("{}", pile.pop().unwrap());
    }
    println!();
}