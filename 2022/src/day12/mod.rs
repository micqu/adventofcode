#![allow(dead_code)]

use std::{collections::{HashMap, BinaryHeap}, cmp::Ordering};

use crate::utils;

pub fn d12_1() {
    let mut start_pos: (i32, i32) = (0, 0);
    let mut end_pos: (i32, i32) = (0, 0);

    let mut map = Vec::<Vec<i32>>::new();
    let mut reader = utils::Reader::load_input("src/day12/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l: Vec::<i32> = line.unwrap().trim().as_bytes().iter()
            .map(|&x| {
                if x.is_ascii_lowercase() {
                    return (x - 97) as i32;
                }
                x as i32
            })
            .collect();
        {
            match l.iter().position(|&x| x == 'S' as i32) {
                Some(idx) => {
                    start_pos = (idx as i32, map.len() as i32);
                }
                None => {},
            }
            match l.iter().position(|&x| x == 'E' as i32) {
                Some(idx) => {
                    end_pos = (idx as i32, map.len() as i32);
                }
                None => {},
            }
        }
        map.push(l);
    }
    
    map[start_pos.1 as usize][start_pos.0 as usize] = 0;
    map[end_pos.1 as usize][end_pos.0 as usize] = 26;

    let start_node = Node {
        cost: 0,
        level: 0,
        position: start_pos,
        prev: None,
    };
    let path = dijkstra(start_node, |n| n.position == end_pos, &map);
    println!("{}", path.len() - 1);
}

pub fn d12_2() {
    let mut start_pos: (i32, i32) = (0, 0);
    let mut map = Vec::<Vec<i32>>::new();

    let mut reader = utils::Reader::load_input("src/day12/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l: Vec::<i32> = line.unwrap().trim().as_bytes().iter()
            .map(|&x| {
                if x.is_ascii_lowercase() {
                    return 26 - (x - 97) as i32;
                }
                x as i32
            })
            .collect();
        {
            match l.iter().position(|&x| x == 'E' as i32) {
                Some(idx) => {
                    start_pos = (idx as i32, map.len() as i32);
                }
                None => {},
            }
        }
        map.push(l);
    }
    
    map[start_pos.1 as usize][start_pos.0 as usize] = 0;

    let start_node = Node {
        cost: 0,
        level: 0,
        position: start_pos,
        prev: None,
    };
    let path = dijkstra(start_node, |n| n.level == 26, &map);
    println!("{}", path.len() - 1);
}

fn dijkstra(start_node: Node, f_goal: impl Fn(Node) -> bool, map: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut goal: Option<Node> = None;
    let mut visited = HashMap::<u64, Node>::new();
    let mut mem = HashMap::<u64, i32>::new();
    let mut q = BinaryHeap::<Node>::new();
    q.push(start_node);
    
    while let Some(v) = q.pop() {
        if f_goal(v) {
            goal = Some(v);
            break;
        }
        
        for mut n in v.get_neighbours(&map) {
            let c = v.cost + n.cost;
            
            match mem.get_mut(&n.get_id()) {
                Some(p) => {
                    if c < *p {
                        *p = c;
                    }
                    continue;
                },
                None => {
                    mem.insert(n.get_id(), c);
                },
            }
            n.cost = c;
            q.push(n);
        }
        
        if !visited.contains_key(&v.get_id()) {
            visited.insert(v.get_id(), v);
        }
    }

    let mut path = Vec::<(i32, i32)>::new();
    while let Some(n) = goal {
        path.push(id_to_pos(n.get_id()));
        match n.prev {
            Some(p) => {
                let node = visited.get(&p);
                goal = Some(*node.unwrap());
            },
            None => {
                break;
            },
        }
    }
    path
}

pub fn id_to_pos(id: u64) -> (i32, i32) {
    let x = id & (i32::MAX as u64);
    let y = (id & ((i32::MAX as u64) << 32)) >> 32;
    (x as i32, y as i32)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Node {
    pub cost: i32,
    pub level: i32,
    pub position: (i32, i32),
    pub prev: Option<u64>,
}

impl Node {
    pub fn get_neighbours(&self, map: &Vec<Vec<i32>>) -> Vec<Node> {
        let h: i32 = map.len() as i32;
        let w: i32 = map[0].len() as i32;

        let adjacent = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        adjacent
            .map(|x| (self.position.0 + x.0, self.position.1 + x.1))
            .iter()
            .filter(|&&x| x.0 >= 0 && x.0 < w && x.1 >= 0 && x.1 < h)
            .map(|&x| {
                Node {
                    cost: 1,
                    level: map[x.1 as usize][x.0 as usize],
                    position: x,
                    prev: Some(self.get_id()),
                }
            })
            .filter(|x| (x.level - self.level) <= 1)
            .collect()
    }

    pub fn get_id(&self) -> u64 {
        self.position.0 as u64 | (self.position.1 as u64) << 32 
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.partial_cmp(&other.cost).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

// for y in 0..h {
//     for x in 0..w {
//         if path.contains(&(x, y)) {
//             print!("*");
//         } else {
//             print!("{}", (map[y as usize][x as usize] as u8 + 97) as char);
//         }
//     }
//     println!();
// }