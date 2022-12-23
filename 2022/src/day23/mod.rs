use std::collections::{HashSet, HashMap};

use crate::utils;

pub fn d23() {
    d23_1_2();
    // d23_1_2_shorter();
}

const FREE: u8 = 0;
const ELF: u8 = 1;
const DIR_X: [i32; 4] = [0, 0, -1, 1];
const DIR_Y: [i32; 4] = [-1, 1, 0, 0];
const ADJ: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
const ADJ_D: [[(i32, i32); 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
    [(-1, 1), (-1, 0), (-1, -1)],
    [(1, -1), (1, 0), (1, 1)],
];
const PADDING: i32 = 100;

pub fn d23_1_2() {
    let l = 75;
    let mut map: Vec<Vec<u8>> = vec![vec![FREE; (PADDING * 2 + l) as usize]; (PADDING * 2 + l) as usize];
    let (mut xmin, mut ymin, mut xmax, mut ymax, n_elves) = parse("src/day23/input.txt", &mut map);

    let mut dir = 0;
    let mut prop: Vec<Vec<u8>> = vec![vec![0; (PADDING * 2 + l) as usize]; (PADDING * 2 + l) as usize];
    let mut to_move = Vec::<(Point, Point)>::with_capacity(n_elves as usize);

    let mut rounds = 0;
    loop {
        rounds += 1;
        for x in xmin as usize..=xmax as usize {
            for y in ymin as usize..=ymax as usize {
                if map[y][x] == ELF && ADJ.iter().any(|c| map[y + c.1 as usize][x + c.0 as usize] == ELF) {
                    for i in 0..4 {
                        let d = (i + dir) % 4;
                        if ADJ_D[d].iter().all(|c| map[(y as i32 + c.1) as usize][(x as i32 + c.0) as usize] == FREE) {
                            prop[(y as i32 + DIR_Y[d]) as usize][(x as i32 + DIR_X[d]) as usize] += 1;
                            let from = Point { x: x as i32, y: y as i32};
                            let to = Point { x: (x as i32) + DIR_X[d], y: y as i32 + DIR_Y[d] };
                            to_move.push((from, to));
                            break;
                        }
                    }
                }
            }
        }

        if to_move.is_empty() {
            break;
        }

        for (from, to) in to_move.iter() {
            if prop[to.y as usize][to.x as usize] == 1 {
                map[to.y as usize][to.x as usize] = ELF;
                map[from.y as usize][from.x as usize] = FREE;
                prop[to.y as usize][to.x as usize] = 0;

                xmin = xmin.min(to.x);
                xmax = xmax.max(to.x);
                ymin = ymin.min(to.y);
                ymax = ymax.max(to.y);
            }
        }
        to_move.clear();

        for a in &mut prop {
            for b in a {
                *b = 0;
            }
        }
        
        dir = (dir + 1) % 4;

        if rounds == 10 {
            let (mut xmin, mut xmax) = (i32::MAX, 0);
            let (mut ymin, mut ymax) = (i32::MAX, 0);
            for i in 0..map.len() {
                for j in 0..map[0].len() {
                    if map[i][j] == ELF {
                        xmin = xmin.min(j as i32);
                        xmax = xmax.max(j as i32);
                        ymin = ymin.min(i as i32);
                        ymax = ymax.max(i as i32);
                    }
                }
            }
            println!("{}", (xmax - xmin + 1) * (ymax - ymin + 1) - n_elves);
        }
    }
    println!("{}", rounds);
}

fn parse(file: &str, elves: &mut Vec<Vec<u8>>) -> (i32, i32, i32, i32, i32) {
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    let (mut xmin, mut xmax) = (i32::MAX, 0);
    let (mut ymin, mut ymax) = (i32::MAX, 0);
    
    let mut n_elves = 0;
    let mut y = 0;
    for line in buffer.lines().filter(|x| !x.is_empty()) {
        let mut x = 0;
        for byte in line.chars() {
            match byte {
                '.' => { },
                '#' => {
                    let nx = x + PADDING;
                    let ny = y + PADDING;

                    xmin = xmin.min(nx);
                    xmax = xmax.max(nx);
                    ymin = ymin.min(ny);
                    ymax = ymax.max(ny);

                    elves[ny as usize][nx as usize] = ELF;
                    n_elves += 1;
                },
                _ => panic!(),
            }
            x += 1;
        }
        y += 1;
    }
    (xmin, ymin, xmax, ymax, n_elves)
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn d23_1_2_shorter() {
    let mut elves = HashSet::<(i32, i32)>::new();
    let n_elves = parse2("src/day23/input.txt", &mut elves);
    let mut prop = HashMap::<(i32, i32), i32>::new();
    let mut dir = 0;
    let mut i = 0;
    loop {
        i += 1;
        let props = elves.iter()
            .map(|&e| {
                if ADJ.iter().any(|c| elves.contains(&(e.0 + c.0, e.1 + c.1))) {
                    for i in 0..4 {
                        let d = (i + dir) % 4;
                        if ADJ_D[d].iter().all(|c| !elves.contains(&(e.0 + c.0, e.1 + c.1))) {
                            *prop.entry((e.0 + DIR_X[d], e.1 + DIR_Y[d])).or_insert(0) += 1;
                            let from = Point { x: e.0, y: e.1};
                            let to = Point { x: e.0 + DIR_X[d], y: e.1 + DIR_Y[d] };
                            return Some((from, to));
                        }
                    }
                }
                None
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        if props.is_empty() {
            println!("{}", i);
            break;
        }
        props.iter().for_each(|(a, b)| {
            if let Some(v) = prop.get(&(b.x, b.y)) {
                if *v == 1 {
                    elves.insert((b.x, b.y));
                    elves.remove(&(a.x, a.y));
                    prop.remove(&(b.x, b.y));
                }
            }
        });

        prop.clear();
        dir = (dir + 1) % 4;

        if i == 10 {
            let xs: Vec<i32> = elves.iter().map(|x| x.0).collect();
            let ys: Vec<i32> = elves.iter().map(|x| x.1).collect();
            println!("{}", (xs.iter().max().unwrap() - xs.iter().min().unwrap() + 1)
                            * (ys.iter().max().unwrap() - ys.iter().min().unwrap() + 1) - n_elves);
            break;
        }
    }
}

fn parse2(file: &str, elves: &mut HashSet<(i32, i32)>) -> i32 {
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    let mut n_elves = 0;
    let mut y = 0;
    for line in buffer.lines().filter(|x| !x.is_empty()) {
        let mut x = 0;
        for byte in line.chars() {
            match byte {
                '.' => { },
                '#' => {
                    elves.insert((x, y));
                    n_elves += 1;
                },
                _ => panic!(),
            }
            x += 1;
        }
        y += 1;
    }
    n_elves
}