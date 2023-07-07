use regex::Regex;
use serde_json::Value;

use crate::utils;

pub fn d12() {
    d12_1();
    d12_2();
}

pub fn d12_1() {
    let r = Regex::new(r"(-?\d+).").unwrap();
    let mut sum = 0;
    let mut reader = utils::Reader::load_input("src/day12/input.txt").unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    buffer.lines().for_each(|line| {
        sum += r
            .captures_iter(line.trim())
            .map(|c| {
                c.iter()
                    .skip(1)
                    .filter_map(|n| n.unwrap().as_str().parse::<i32>().ok())
                    .sum::<i32>()
            })
            .sum::<i32>();
    });
    println!("{}", sum);
}

pub fn d12_2() {
    let mut reader = utils::Reader::load_input("src/day12/input.txt").unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);

    let k: Value = serde_json::from_str(&buffer.to_owned()).unwrap();
    let sum = solve(&k);
    println!("{}", sum);
}

fn solve(k: &Value) -> i64 {
    if let Some(n) = k.as_i64() {
        return n;
    }
    if let Some(ko) = k.as_object() {
        if ko.iter().any(|(_, v)| v == "red") {
            return 0;
        }
        return ko.iter().map(|(_, x)| solve(x)).sum::<i64>();
    }
    if let Some(ka) = k.as_array() {
        return ka.iter().map(|x| solve(x)).sum::<i64>();
    }
    0
}
