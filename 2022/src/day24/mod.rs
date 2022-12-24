use core::panic;
use std::collections::{HashSet, HashMap, BinaryHeap};

use crate::utils;

pub fn d24() {
    d24_1_2();
}

fn d24_1_2() {
    let mut bsm = HashMap::<u32, Vec<Blizzard>>::new();
    let mut bsa: Vec<Blizzard> = Vec::new();
    let (h, w) = parse("src/day24/input.txt", &mut bsa);

    for t in 0..((h as usize - 2)*(w as usize - 2)) as u32 {
        bsa.iter_mut().for_each(|x| x.blow_mut(w, h));
        bsm.entry(t).or_insert(bsa.clone());
    }

    let start = Visit { me: Me { x: 1, y: 0 }, time: 0 };
    let result = search(start, |u| u.me.x == w - 2 && u.me.y == h - 1, &bsm, w, h).unwrap();
    println!("{}", result.time + 1);

    let start = Visit { me: Me { x: w - 2, y: h - 1 }, time: result.time + 1 };
    let result = search(start, |u| u.me.x == 1 && u.me.y == 0, &bsm, w, h).unwrap();

    let start = Visit { me: Me { x: 1, y: 0 }, time: result.time + 1 };
    let result = search(start, |u| u.me.x == w - 2 && u.me.y == h - 1, &bsm, w, h).unwrap();
    println!("{}", result.time + 1);
}

fn search(start: Visit, goal: impl Fn(&Visit) -> bool, bsm: &HashMap::<u32, Vec<Blizzard>>, w: i8, h: i8) -> Option<Visit> {
    let mut visited = HashSet::<Visit>::new();
    let mut q = BinaryHeap::<Visit>::new();
    q.push(start);

    while let Some(u) = q.pop() {
        if goal(&u) {
            return Some(u);
        }
        
        let nbs = bsm.get(&((u.time + 1) % bsm.len() as u32)).unwrap();
        const ADJ: [(i8, i8); 5] = [(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)];
        for adj in ADJ
            .map(|c| (u.me.x + c.0, u.me.y + c.1))
            .iter()
            .filter(|c| {
                if (c.0 == 1 && c.1 == 0) || (c.0 == w - 2 && c.1 == h - 1) {
                    return true;
                }
                c.0 > 0 && c.0 < w - 1 && c.1 > 0 && c.1 < h - 1
            }) {
                
            if nbs.iter().any(|c| c.x == adj.0 && c.y == adj.1) {
                continue;
            }

            let n = Visit {
                me: Me {
                    x: adj.0,
                    y: adj.1,
                },
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
    me: Me,
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Clone)]
struct Me {
    x: i8,
    y: i8,
}