use std::{collections::{HashMap, VecDeque, HashSet}, fmt::Debug};

use crate::utils;

pub fn d16() {
    d16_1();
    // d16_2();
}

pub fn d16_1() {
    let mut valves = Vec::<Valve>::new();
    let connections = parse("src/day16/input.txt", &mut valves);
    let distances = get_distances(&connections);
    let start = *connections.keys().find(|x| x.name == "AA").unwrap();
    let remaining = connections.keys().filter(|x| x.flow > 0).map(|x| *x).collect();
    let time = 30;

    let (_, pressure) = solve(start, &connections, &distances, &remaining, time);
    // dbg!(p);
    println!("{}", pressure);
}

pub fn d16_2() {
    let mut valves = Vec::<Valve>::new();
    let connections = parse("src/day16/input.txt", &mut valves);
    let distances = get_distances(&connections);
    let start = *connections.keys().find(|x| x.name == "AA").unwrap();
    let remaining = connections.keys().filter(|x| x.flow > 0).map(|x| *x).collect();
    let time = 26;
    //2189
    // let ps = solve(start, &connections, &distances, &remaining, time);
    // let remaining = remaining.iter().filter(|x| !ps.0.contains(x)).map(|x| *x).collect::<Vec<&Valve>>();
    // let ps2 = solve(start, &connections, &distances, &remaining, time);
    // dbg!(ps.1 + ps2.1);

    let ps = solve2(start, &connections, &distances, &remaining, time);
    dbg!(ps.len());
    
    let mut new_ps = Vec::<(Vec<&Valve>, Vec<&Valve>, u32)>::new();
    for i in 0..ps.len() - 1 {
        let a = &ps[i].0.iter().filter(|x| x.name != "AA").collect::<HashSet<_>>();
        
        for j in i + 1..ps.len() {
            let b = &ps[j].0.iter().filter(|x| x.name != "AA").collect::<HashSet<_>>();

            if a.intersection(&b).count() == 0 {
                new_ps.push((
                    ps[i].0.clone(),
                    ps[j].0.clone(),
                    ps[i].1 + ps[j].1)
                );
            }
        }
    }
    new_ps.sort_by(|a, b| b.2.cmp(&a.2));
    dbg!(new_ps.len());
    let best = new_ps.iter().next().unwrap();
    dbg!(best);
}

fn solve<'a>(
    start: &'a Valve,
    connections: &'a HashMap::<&Valve, Vec<&'a Valve>>,
    distances: &'a HashMap::<String, HashMap::<String, u32>>,
    remaining: &Vec<&'a Valve>,
    time: u32) -> (Vec<&'a Valve>, u32) {
    
    let mut paths = Vec::<(Vec<&Valve>, u32)>::new();
    for n in remaining {
        let d = distances[&start.name][&n.name];
        if d >= time {
            continue;
        }
        
        let time_left = time - d - 1;
        let remaining = remaining.iter().filter(|x| *x != n).map(|x| *x).collect();
        let child = solve(n, connections, distances, &remaining, time_left);
        
        let mut current_path = vec![start];
        current_path.extend(child.0);
        paths.push((current_path, n.flow * time_left + child.1))
    }

    paths.sort_by(|a, b| b.1.cmp(&a.1));
    if let Some(k) = paths.first() {
        return k.clone();
    }
    return (vec![start], 0);
}

fn solve2<'a>(
    start: &'a Valve,
    connections: &'a HashMap::<&Valve, Vec<&'a Valve>>,
    distances: &'a HashMap::<String, HashMap::<String, u32>>,
    remaining: &Vec<&'a Valve>,
    time: u32
) -> Vec<(Vec<&'a Valve>, u32)> {

    let mut paths = Vec::<(Vec<&Valve>, u32)>::new();
    for n in remaining {
        let d = distances[&start.name][&n.name];
        if d >= time {
            continue;
        }

        let time_left = time - d - 1;
        let remaining = remaining.iter().filter(|x| *x != n).map(|x| *x).collect();
        let children = solve2(n, connections, distances, &remaining, time_left);
        
        if children.is_empty() {
            paths.push((vec![start, n], n.flow * time_left));
            continue;
        }

        for child in children {
            let mut current_path = vec![start];
            current_path.extend(child.0);
            paths.push((current_path, n.flow * time_left + child.1));
        }
    }
    
    paths
}

fn parse<'a>(file: &'a str, valves: &'a mut Vec<Valve>) -> HashMap::<&'a Valve, Vec<&'a Valve>> {
    let mut edges = HashMap::<String, Vec<String>>::new();
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    buffer
        .lines()
        .for_each(|line| {
            let tokens = line.split_terminator(&[' ', ';', ',', '='])
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
            
            let name = tokens[1].to_owned();
            
            valves.push(Valve {
                name: name.clone(),
                flow: tokens[5].parse().unwrap(),
            });

            edges.insert(name, tokens
                .iter()
                .skip(10)
                .map(|&x| x.to_owned()).collect()
            );
        });
        
    let mut neighbours = HashMap::<&Valve, Vec<&Valve>>::new();
    for valve in valves.iter() {
        for edge_name in edges.get(&valve.name).unwrap() {
            let n = valves.iter().find(|x| x.name == *edge_name).unwrap();
            neighbours.entry(valve).or_insert(Vec::new()).push(n);
        }
    }
    neighbours
}

fn get_distances<'a>(valves: &'a HashMap::<&Valve, Vec<&Valve>>) -> HashMap::<String, HashMap::<String, u32>> {
    let mut distances_all = HashMap::<String, HashMap::<String, u32>>::new();
    for valve in valves.keys() {
        let distances = distances_all.entry(valve.name.clone()).or_insert(HashMap::new());
        
        let mut q = VecDeque::<Visit>::new();
        q.push_back(Visit {
            valve: &valve,
            distance: 0
        });
        
        let mut visited = HashSet::<&Valve>::new();
        while let Some(Visit { valve, distance }) = q.pop_front() {
            if !visited.insert(valve) {
                continue;
            }

            if let Some(ns) = valves.get(valve) {
                for n in ns {
                    let c = distance + 1;
                    if let Some(p) = distances.get_mut(&n.name) {
                        if c < *p {
                            *p = c;
                        }
                        continue;
                    }
                    q.push_back(Visit { valve: n, distance: c });
                    distances.insert(n.name.clone(), c);
                }
            }
        }
    }
    distances_all
}

pub struct Visit<'a> {
    valve: &'a Valve,
    distance: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Valve {
    name: String,
    flow: u32
}



/*
best = (
    [
        Valve {
            name: "AA",
            flow: 0,
        },
        Valve {
            name: "CD",
            flow: 6,
        },
        Valve {
            name: "ZB",
            flow: 9,
        },
        Valve {
            name: "XG",
            flow: 18,
        },
        Valve {
            name: "RG",
            flow: 17,
        },
        Valve {
            name: "CE",
            flow: 21,
        },
        Valve {
            name: "OU",
            flow: 10,
        },
    ],
    [
        Valve {
            name: "AA",
            flow: 0,
        },
        Valve {
            name: "GF",
            flow: 19,
        },
        Valve {
            name: "EK",
            flow: 20,
        },
        Valve {
            name: "AW",
            flow: 11,
        },
        Valve {
            name: "YQ",
            flow: 14,
        },
        Valve {
            name: "XR",
            flow: 7,
        },
        Valve {
            name: "DT",
            flow: 5,
        },
        Valve {
            name: "TM",
            flow: 3,
        },
    ],
    2189,
)
*/