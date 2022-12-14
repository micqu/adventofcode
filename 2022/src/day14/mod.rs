use std::collections::{VecDeque, HashSet};

use crate::utils;

pub fn d14_1() {
    let mut map = parse("src/day14/input.txt");
    let mut count = 0;
    let floor = 171;
    loop {
        let top = map.iter()
            .map(|x| x[500])
            .position(|x| x == 1)
            .unwrap() as i32;
        let mut sand: (i32, i32) = (500, top - 1);
        loop {            
            let last = sand;
            let new_y = (sand.1 + 1) as usize;
            
            if new_y == floor {
                println!("{}", count);
                return;
            }
            
            if map[new_y][sand.0 as usize] == 0 {
                sand.1 += 1;
            } else if map[new_y][(sand.0 - 1) as usize] == 0 {
                sand.0 -= 1;
                sand.1 += 1;
            } else if map[new_y][(sand.0 + 1) as usize] == 0 {
                sand.0 += 1;
                sand.1 += 1;
            }
            
            if sand == last {
                break;
            }
        }
    
        map[sand.1 as usize][sand.0 as usize] = 1;
        count += 1;
        if sand.1 == 0 {
            break;
        }
    }
    println!("{}", count);
}

pub fn d14_2() {
    let map = parse("src/day14/input.txt");
    let start_node = Node { position: (500, 0) };
    let count = bfs(start_node, |n| n.position.1 == 171, &map);
    println!("{}", count);
}

fn parse(file: &str) -> Vec::<Vec<i32>> {
    let mut s = vec![vec![0; 1000]; 172];
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        line.unwrap()
            .split("->")
            .map(|x| x
                .trim()
                .split(',')
                .collect::<Vec<&str>>()
            )
            .map(|x| (x[0].parse::<i32>().unwrap(), x[1].parse::<i32>().unwrap()))
            .collect::<Vec::<_>>()
            .windows(2)
            .for_each(|x| {
                let dx = x[1].0 - x[0].0;
                if dx > 0 {
                    for i in 0..=dx {
                        s[x[0].1 as usize][(x[0].0 + i) as usize] = 1;
                    }
                } else {
                    for i in (0..=-dx).rev() {
                        s[x[0].1 as usize][(x[0].0 - i) as usize] = 1;
                    }
                }
                let dy = x[1].1 - x[0].1;
                if dy > 0 {
                    for i in 0..=dy {
                        s[(x[0].1 + i) as usize][x[0].0 as usize] = 1;
                    }
                } else {
                    for i in (0..=-dy).rev() {
                        s[(x[0].1 - i) as usize][x[0].0 as usize] = 1;
                    }
                }
            });
    }
    s
}

fn bfs(start_node: Node, f_goal: impl Fn(&Node) -> bool, map: &Vec<Vec<i32>>) -> usize {
    let mut c = 0;
    let mut mem = HashSet::<u64>::new();
    let mut q = VecDeque::<Node>::new();
    q.push_back(start_node);
    while let Some(v) = q.pop_front() {
        if f_goal(&v) {
            break;
        }
        c += 1;
        for n in v.get_neighbours(&map) {
            let id = n.get_id();
            if !mem.contains(&id) {
                mem.insert(id);
                q.push_back(n);
            }
        }
    }
    c
}

struct Node {
    pub position: (i32, i32),
}

impl Node {
    pub fn get_neighbours(&self, map: &Vec<Vec<i32>>) -> Vec<Node> {
        [(0, 1), (-1, 1), (1, 1)]
            .map(|x| (self.position.0 + x.0, self.position.1 + x.1))
            .iter()
            .filter(|&&sand|
                map[sand.1 as usize][sand.0 as usize] == 0
                || map[sand.1 as usize][sand.0 as usize] == 0
                || map[sand.1 as usize][sand.0 as usize] == 0
            )
            .map(|&x| Node { position: x })
            .collect()
    }

    pub fn get_id(&self) -> u64 {
        self.position.0 as u64 | (self.position.1 as u64) << 32 
    }
}