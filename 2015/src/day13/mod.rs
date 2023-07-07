use std::{collections::HashMap};

use crate::utils;

pub fn d13() {
    d13_1();
    d13_2();
}

pub fn d13_1() {
    let mut edges: HashMap<String, HashMap<String, i32>> = HashMap::new();
    parse("src/day13/input.txt", &mut edges);
    let start = edges.keys().next().unwrap();
    let h = solve(start, &vec![start], &edges);
    println!("{}", h);
}

pub fn d13_2() {
    let mut edges: HashMap<String, HashMap<String, i32>> = HashMap::new();
    parse("src/day13/input.txt", &mut edges);
    
    let mut a = HashMap::new();
    edges.iter_mut().for_each(|(k, v)| {
        v.insert("Me".to_string(), 0);
        a.insert(k.to_string(), 0);
    });
    edges.insert("Me".to_string(), a);
    
    let start = edges.keys().next().unwrap();
    let h = solve(start, &vec![start], &edges);
    println!("{}", h);
}

fn solve(start: &String, visited: &Vec<&String>, edges: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut h = 0;

    if visited.len() == edges.keys().len() {
        let f = visited.iter().next().unwrap();
        return edges[start][*f] + edges[*f][start];
    }

    for n in edges[start].keys() {
        if visited.contains(&n) {
            continue;
        }

        let mut nv = visited.clone();
        nv.push(n);
        h = h.max(edges[start][n] + edges[n][start] + solve(n, &nv, edges));
    };
    h
}

fn parse(file: &str, edges: &mut HashMap<String, HashMap<String, i32>>) {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    buffer.lines().for_each(|line| {
        let tokens: Vec<&str> = line.trim().split(&[' ', '.']).collect();
        let a = tokens[0].to_string();
        let b = tokens[10].to_string();
        let h = match tokens[2] {
            "gain" => tokens[3].parse::<i32>().unwrap(),
            "lose" => -tokens[3].parse::<i32>().unwrap(),
            _ => panic!()
        };
        edges.entry(a).or_insert(HashMap::new()).insert(b, h);
    });
}