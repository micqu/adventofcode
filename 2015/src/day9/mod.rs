use std::collections::{HashMap, HashSet};

use crate::utils;

pub fn d9() {
    let mut edges: HashMap<String, HashMap<String, i32>> = HashMap::new();
    parse("src/day9/input.txt", &mut edges);
    d9_1(&edges.clone());
    d9_2(&edges);
}

pub fn d9_1(edges: &HashMap<String, HashMap<String, i32>>) {
    let mut c = Vec::<Option<i32>>::new();
    for key in edges.keys() {
        let mut visited = HashSet::new();
        visited.insert(key);
        c.push(solve(key, &visited, &edges));
    }
    let r = c.iter().filter_map(|x| *x).min().unwrap();
    println!("{}", r);
}

pub fn d9_2(edges: &HashMap<String, HashMap<String, i32>>) {
    let mut c = Vec::<Option<i32>>::new();
    for key in edges.keys() {
        let mut visited = HashSet::new();
        visited.insert(key);
        c.push(solve2(key, &visited, &edges));
    }
    let r = c.iter().filter_map(|x| *x).max().unwrap();
    println!("{}", r);
}

fn solve(start: &String, visited: &HashSet<&String>, edges: &HashMap<String, HashMap<String, i32>>) -> Option<i32> {
    let mut c = Vec::<i32>::new();

    if edges[start].keys().all(|x| visited.contains(x)) {
        return Some(0);
    }

    for r in edges[start].keys() {
        if visited.contains(r) {
            continue;
        }

        let mut k = visited.clone();
        k.insert(r);

        if let Some(n) = solve(r, &k, edges) {
            c.push(edges[start][r] + n);
        }
    }

    if let Some(n) = c.iter().min() {
        return Some(*n);
    }
    None
}

fn solve2(start: &String, visited: &HashSet<&String>, edges: &HashMap<String, HashMap<String, i32>>) -> Option<i32> {
    let mut c = Vec::<i32>::new();

    if edges[start].keys().all(|x| visited.contains(x)) {
        return Some(0);
    }

    for r in edges[start].keys() {
        if visited.contains(r) {
            continue;
        }

        let mut k = visited.clone();
        k.insert(r);

        if let Some(n) = solve2(r, &k, edges) {
            c.push(edges[start][r] + n);
        }
    }

    if let Some(n) = c.iter().max() {
        return Some(*n);
    }
    None
}

fn parse(file: &str, edges: &mut HashMap<String, HashMap<String, i32>>) {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    buffer.lines().for_each(|line| {
        let tokens: Vec<&str> = line.trim().split(" ").collect();
        let from = tokens[0].to_string();
        let to = tokens[2].to_string();
        let v: i32 = tokens[4].parse().unwrap();
        edges.entry(from.clone()).or_insert(HashMap::new()).insert(to.clone(), v);
        edges.entry(to).or_insert(HashMap::new()).insert(from, v);
    });
}