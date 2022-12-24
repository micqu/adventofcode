use core::panic;
use std::{collections::{HashSet, HashMap, BinaryHeap}, hash::Hash, time::SystemTime};

use crate::utils;

pub fn d24() {
    d24_1_2();
}

fn d24_1_2() {
    let mut bsm = HashMap::<u32, HashMap<(i8, i8), Blizzard>>::new();
    let mut bsa: Vec<Blizzard> = Vec::new();
    let (h, w) = parse("src/day24/input.txt", &mut bsa);

    bsm.insert(0, bsa.iter().map(|v| ((v.x, v.y), *v)).collect());
    for t in 1..((h as u32 - 2) * (w as u32 - 2)) as u32 {
        bsa.iter_mut().for_each(|v| v.blow_mut(w, h));
        bsm.entry(t).or_insert(bsa.iter().map(|x| ((x.x, x.y), *x)).collect());
    }

    let time = SystemTime::now();
    let start = Visit { x: 1, y: 0, time: 0 };
    let result = search(start, |u| u.x == w - 2 && u.y == h - 1, &bsm, w, h).unwrap();
    println!("{}", result.time);

    let start = Visit { x: w - 2, y: h - 1, time: result.time };
    let result = search(start, |u| u.x == 1 && u.y == 0, &bsm, w, h).unwrap();

    let start = Visit { x: 1, y: 0, time: result.time };
    let result = search(start, |u| u.x == w - 2 && u.y == h - 1, &bsm, w, h).unwrap();
    println!("{}", result.time);

    println!("Travel time: {:?}", time.elapsed().unwrap());
}

fn search(
    start: Visit,
    goal: impl Fn(&Visit) -> bool,
    bsm: &HashMap::<u32, HashMap<(i8, i8), Blizzard>>,
    w: i8,
    h: i8
) -> Option<Visit> {
    let mut visited = HashSet::<Visit>::new();
    let mut q = BinaryHeap::<Visit>::new();
    q.push(start);

    while let Some(u) = q.pop() {
        if goal(&u) {
            return Some(u);
        }
        
        const ADJ: [(i8, i8); 5] = [(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)];
        let nbs = bsm.get(&((u.time + 1) % bsm.len() as u32)).unwrap();
        
        for adj in ADJ.iter()
            .filter_map(|a| {
                let n = (u.x + a.0, u.y + a.1);
                (
                    (n.0 > 0 && n.0 < w - 1 && n.1 > 0 && n.1 < h - 1)
                    || (n.0 == 1 && n.1 == 0)
                    || (n.0 == w - 2 && n.1 == h - 1)
                ).then_some(n)
            }) {

            if let Some(_) = nbs.get(&(adj.0, adj.1)) {
                continue;
            }

            let n = Visit {
                x: adj.0,
                y: adj.1,
                time: u.time + 1,
            };

            if visited.insert(n.clone()) {
                q.push(n);
            }
        }
    }
    None
}

fn parse<'a>(file: &str, blizzards: &'a mut Vec<Blizzard>) -> (i8, i8) {
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    let mut w = 0;
    let mut y = 0;

    for line in buffer.lines().filter(|x| !x.is_empty()) {
        let mut x = 0;

        for byte in line.chars() {
            if byte == '.' || byte == '#' {
                x += 1;
                continue;
            }

            let dir = match byte {
                '>' => 0,
                'v' => 1,
                '<' => 2,
                '^' => 3,
                _ => panic!(),
            };
            blizzards.push(Blizzard { x, y, dir });
            x += 1;
        }
        w = w.max(x);
        y += 1;
    }
    (y, w)
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Visit {
    x: i8,
    y: i8,
    time: u32,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy, Hash)]
struct Blizzard {
    x: i8,
    y: i8,
    dir: usize,
}

const DIR_X: [i8; 4] = [1, 0, -1, 0];
const DIR_Y: [i8; 4] = [0, 1, 0, -1];

impl Blizzard {
    fn blow_mut(&mut self, w: i8, h: i8) {
        let mut nx = self.x + DIR_X[self.dir];
        let mut ny = self.y + DIR_Y[self.dir];
        if nx < 1 {
            nx = w - 2;
        }
        if nx > w - 2 {
            nx = 1;
        }
        if ny < 1 {
            ny = h - 2;
        }
        if ny > h - 2 {
            ny = 1;
        }
        self.x = nx;
        self.y = ny;
    }
}