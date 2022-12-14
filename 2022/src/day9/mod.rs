use core::panic;
use std::collections::HashSet;

use crate::utils;

pub fn d9() {
    d9_1();
    d9_2();
}

pub fn d9_1() {
    let mut h: (i32, i32) = (0, 0);
    let mut t: (i32, i32) = (0, 0);
    let mut map = HashSet::<(i32, i32)>::new();
    map.insert(t);

    let mut reader = utils::Reader::load_input("src/day9/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let cmd = line.unwrap().trim().split_whitespace().collect::<Vec<&str>>();

        for _ in 0..cmd[1].parse().unwrap() {
            match cmd[0].chars().next().unwrap() {
                'U' => {
                    h.1 += 1;
                    if chebyshev(h, t) > 1 {
                        t = (h.0, h.1 - 1);
                    }
                },
                'D' => {
                    h.1 -= 1;
                    if chebyshev(h, t) > 1 {
                        t = (h.0, h.1 + 1);
                    }
                },
                'L' => {
                    h.0 -= 1;
                    if chebyshev(h, t) > 1 {
                        t = (h.0 + 1, h.1);
                    }
                },
                'R' => {
                    h.0 += 1;
                    if chebyshev(h, t) > 1 {
                        t = (h.0 - 1, h.1);
                    }
                },
                _ => panic!()
            }
            map.insert(t);
        }
    }
    println!("{}", map.len());
}

pub fn d9_2() {
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut map = HashSet::<(i32, i32)>::new();
    map.insert(*knots.last().unwrap());

    let mut reader = utils::Reader::load_input("src/day9/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let cmd = line.unwrap().trim().split_whitespace().collect::<Vec<&str>>();

        for _ in 0..cmd[1].parse().unwrap() {
            match cmd[0].chars().next().unwrap() {
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                'L' => knots[0].0 -= 1,
                'R' => knots[0].0 += 1,
                _ => panic!()
            }

            for i in 0..knots.len() - 1 {
                let d = d1(knots[i + 1], knots[i]);
                if d.0.abs() > 1 || d.1.abs() > 1 {
                    knots[i + 1].0 += d.0.signum();
                    knots[i + 1].1 += d.1.signum();

                    if i == knots.len() - 2 {
                        map.insert(*knots.last().unwrap());
                    }
                    continue;
                }
                break;
            };
        }
    }
    println!("{}", map.len());
}

fn chebyshev(a: (i32, i32), b: (i32, i32)) -> i32 {
    let dx = (b.0 - a.0).abs();
    let dy = (b.1 - a.1).abs();
    i32::max(dx, dy)
}

fn d1(from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    (dx, dy)
}