use std::collections::HashSet;

use crate::utils;

pub fn d18() {
    d18_1();
    d18_2();
}

pub fn d18_1() {
    let mut map: HashSet<Point> = HashSet::new();
    let (h, w) = parse("src/day18/input.txt", &mut map);
    
    const ADJ: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    let mut tmp: HashSet<Point> = HashSet::new();

    for _ in 0..100 {
        tmp.clear();

        for i in 0..h {
            for j in 0..w {
                let nc = ADJ.iter().filter(|(a, b)| map.contains(&Point { x: j + a, y: i + b })).count();
                let curr = Point { x: j, y: i };
    
                if let Some(p) = map.get(&curr) {
                    if nc == 2 || nc == 3 {
                        tmp.insert(p.clone());
                    }
                } else {
                    if nc == 3 {
                        tmp.insert(curr);
                    }
                }
            }
        }
        map.clear();
        map.extend(tmp.iter().cloned());
    }
    println!("{}", map.len())
}

pub fn d18_2() {
    let mut map: HashSet<Point> = HashSet::new();
    let (h, w) = parse("src/day18/input.txt", &mut map);
    
    const ADJ: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    let mut tmp: HashSet<Point> = HashSet::new();

    map.extend([
        Point { x: 0, y: 0 },
        Point { x: w - 1, y: 0 },
        Point { x: 0, y: h - 1 },
        Point { x: w - 1, y: h - 1 },
    ]);

    for _ in 0..100 {
        tmp.clear();

        for i in 0..h {
            for j in 0..w {
                let nc = ADJ.iter().filter(|(a, b)| map.contains(&Point { x: j + a, y: i + b })).count();
                let curr = Point { x: j, y: i };
    
                if let Some(p) = map.get(&curr) {
                    if nc == 2 || nc == 3 {
                        tmp.insert(p.clone());
                    }
                } else {
                    if nc == 3 {
                        tmp.insert(curr);
                    }
                }
            }
        }
        map.clear();
        map.extend(tmp.iter().cloned());
        map.extend([
            Point { x: 0, y: 0 },
            Point { x: w - 1, y: 0 },
            Point { x: 0, y: h - 1 },
            Point { x: w - 1, y: h - 1 },
        ]);
    }
    println!("{}", map.len())
}

fn parse(file: &str, map: &mut HashSet<Point>) -> (i32, i32) {
    let mut w = 0;
    let mut h = 0;
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    buffer.lines().enumerate().for_each(|(y, line)| {
        for (x, ch) in line.char_indices() {
            if ch == '#' {
                let p = Point {
                    x: x as i32,
                    y: y as i32,
                };
                map.insert(p);
            }
        }
        h = h.max(y + 1);
        w = line.len();
    });
    (h as i32, w as i32)
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}