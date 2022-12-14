use core::panic;
use std::collections::HashMap;

use crate::utils;

pub fn d7() {
    d7_1();
    d7_2();
}

pub fn d7_1() {
    let map = parse();
    let mut sorted = map.iter().collect::<Vec<_>>();
    sorted.sort();
    sorted.reverse();

    let mut sums = HashMap::<String, u32>::new();

    for (path, numbers) in sorted {
        let sum: u32 = numbers.iter().sum();
        sums.entry(path.to_owned()).or_insert(sum);
    }

    let mut agg = HashMap::<String, u32>::new();
    for (path, sum) in sums {
        // println!("Path: {}", path);
        
        let mut tokens = path.split_terminator('/').collect::<Vec<&str>>();
        let mut parent: Option<&str>;
        while tokens.len() > 0 {
            parent = tokens.last().copied();
            match parent {
                Some(_) => {
                    let pathp = tokens.join("/");
                    let entry = agg.entry(pathp).or_insert(0);
                    *entry += sum;
                },
                None => {
                    let entry = agg.entry("/".to_owned()).or_insert(0);
                    *entry += sum;
                },
            }
            tokens.pop();
        }
    }
    let result: u32 = agg.iter().filter(|a| *a.1 < 100000).map(|a| *a.1).sum();
    println!("{}", result);
}

pub fn d7_2() {
    let map = parse();
    let mut sorted = map.iter().collect::<Vec<_>>();
    sorted.sort();
    sorted.reverse();

    let mut sums = HashMap::<String, u32>::new();

    for (path, numbers) in sorted {
        let sum: u32 = numbers.iter().sum();
        sums.entry(path.to_owned()).or_insert(sum);
    }

    let mut agg = HashMap::<String, u32>::new();
    for (path, sum) in sums {
        // println!("Path: {}", path);
        
        let mut tokens = path.split_terminator('/').collect::<Vec<&str>>();
        let mut parent: Option<&str>;
        while tokens.len() > 0 {
            parent = tokens.last().copied();
            match parent {
                Some(_) => {
                    let pathp = tokens.join("/");
                    let entry = agg.entry(pathp).or_insert(0);
                    *entry += sum;
                },
                None => {
                    let entry = agg.entry("/".to_owned()).or_insert(0);
                    *entry += sum;
                },
            }
            tokens.pop();
        }
    }
    
    let exceeded = agg.get("").unwrap() - 40000000;
    let mut result: Vec<u32> = agg
        .iter()
        .filter(|a| *a.1 >= exceeded)
        .map(|a| *a.1)
        .collect::<Vec<u32>>();
    result.sort();
    println!("{}", result.first().unwrap());
}

fn parse() -> HashMap::<String, Vec<u32>> {
    let mut map = HashMap::<String, Vec<u32>>::new();
    let mut pwd: Vec<String> = Vec::new();
    let mut ls = false;

    let mut reader = utils::Reader::load_input("src/day7/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let tokens = line
            .unwrap()
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>();
        
        if ls {
            match tokens[0] {
                "dir" => {},
                _ => {
                    let parsed = tokens[0].parse::<u32>();
                    match parsed {
                        Ok(p) => {
                            let mut path: String = "/".to_owned();
                            if pwd.len() >= 2 {
                                path = "/".to_owned() + &pwd[2..].join("/");
                            }
                            map.entry(path)
                                .or_insert(vec![])
                                .push(p);
                        },
                        Err(_) => ls = false,
                    }
                },
            }
        }

        if !ls && tokens[0] == "$" {
            match tokens[1] {
                "cd" => {                        
                    match tokens[2] {
                        ".." => {
                            pwd.pop();
                        },
                        _ => {
                            if tokens[2].as_bytes()[0] == '/' as u8 {
                                pwd.clear();
                                pwd.push("/".to_owned());
                            }
                            
                            let mut path = tokens[2]
                                .split_terminator("/")
                                .map(|a| a.to_owned())
                                .collect::<Vec<String>>();
                            pwd.append(&mut path);
                        }
                    }
                },
                "ls" => {
                    ls = true;
                    continue;
                }
                _ => panic!()
            }
        }
    }
    map
}