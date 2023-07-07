use std::collections::{HashMap, HashSet};

use crate::utils;

pub fn d19() {
    d19_1();
    d19_2();
}

pub fn d19_1() {
    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
    let molecule = parse("src/day19/input.txt", &mut replacements);
    let mut mem = HashSet::<String>::new();
    for (key, val) in replacements {
        for v in val {
            for (p, _) in molecule.match_indices(&key) {
                let mut rep = molecule.clone();
                rep.replace_range(p..p+key.len(), &v);
                mem.insert(rep);
            }
        }
    }
    println!("{}", mem.len());
}

pub fn d19_2() {
    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
    let molecule = parse("src/day19/input.txt", &mut replacements);
    
    let mut freq = HashMap::<String, usize>::new();
    let n = ["Rn", "Ar", "Y"].map(|x| x.to_string());
    let mut ks: Vec<_> = replacements.keys().collect();
    ks.extend(&n);
    for key in ks {
        let m = molecule.match_indices(key);
        freq.insert(key.to_string(), m.count());
    }
    let a = freq.values().sum::<usize>();
    let b = freq["Rn"] + freq["Ar"];
    let c = freq["Y"];
    println!("{}", a - b - 2 * c - 1);
}

fn parse(file: &str, replacements: &mut HashMap<String, Vec<String>>) -> String {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    for line in buffer.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains("=>") {
            let tokens: Vec<_> = line.trim().split(' ').collect();
            let from = tokens[0].to_string();
            let to = tokens[2].to_string();
            replacements.entry(from).or_insert(Vec::new()).push(to);
            continue;
        }

        return line.to_string();
    }
    String::new()
}