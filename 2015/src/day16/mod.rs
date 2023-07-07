use std::collections::HashMap;

use crate::utils;

pub fn d16() {
    d16_1();
    d16_2();
}

pub fn d16_1() {
    let mut sues: Vec<Sue> = Vec::new();
    parse("src/day16/input.txt", &mut sues);

    let mut id: HashMap<String, u32> = HashMap::new();
    id.insert("children".to_string(), 3);
    id.insert("cats".to_string(), 7);
    id.insert("samoyeds".to_string(), 2);
    id.insert("pomeranians".to_string(), 3);
    id.insert("akitas".to_string(), 0);
    id.insert("vizslas".to_string(), 0);
    id.insert("goldfish".to_string(), 5);
    id.insert("trees".to_string(), 3);
    id.insert("cars".to_string(), 2);
    id.insert("perfumes".to_string(), 1);

    for (i, sue) in sues.iter().enumerate() {
        if sue.items.iter().all(|(k, v)| id[k] == *v) {
            println!("{}", i + 1);
        }
    }
}

pub fn d16_2() {
    let mut sues: Vec<Sue> = Vec::new();
    parse("src/day16/input.txt", &mut sues);

    let mut id: HashMap<String, u32> = HashMap::new();
    id.insert("children".to_string(), 3);
    id.insert("cats".to_string(), 7);
    id.insert("samoyeds".to_string(), 2);
    id.insert("pomeranians".to_string(), 3);
    id.insert("akitas".to_string(), 0);
    id.insert("vizslas".to_string(), 0);
    id.insert("goldfish".to_string(), 5);
    id.insert("trees".to_string(), 3);
    id.insert("cars".to_string(), 2);
    id.insert("perfumes".to_string(), 1);

    for (i, sue) in sues.iter().enumerate() {
        if sue.items.iter().all(|(k, v)| {
            match k.as_str() {
                "cats" | "trees" => id[k] < *v,
                "pomeranians" | "goldfish" => id[k] > *v,
                _ => id[k] == *v,
            }
        }) {
            println!("{}", i + 1);
        }
    }
}

fn parse(file: &str, sues: &mut Vec<Sue>) {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    buffer.lines().for_each(|line| {
        let tokens: Vec<&str> = line.trim().split(&[':', ' ', ',']).skip(3).collect();
        let mut items = HashMap::new();
        for chunk in tokens.chunks(4) {
            items.insert(chunk[0].to_string(), chunk[2].parse().unwrap());
        }
        sues.push(Sue {
            items: items,
        });
    });
}

#[derive(Debug)]
struct Sue {
    items: HashMap<String, u32>,
}