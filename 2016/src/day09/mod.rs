use itertools::Itertools;

use crate::{IntoSolution, Solution};

pub const TITLE: &str = "Explosives in Cyberspace";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut markers = Vec::<Marker>::new();
    let mut c: isize = 0;
    let mut in_marker = false;
    let mut str = String::new();
    let mut skip: usize = 0;

    for ch in INPUT.chars() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        if ch == '(' {
            in_marker = true;
            continue;
        }

        if ch == ')' {
            let split = str.split('x').collect_vec();
            let marker = Marker {
                len: split[0].parse().unwrap(),
                n: split[1].parse().unwrap(),
            };
            skip = marker.len;
            markers.push(marker);
            
            str.clear();
            in_marker = false;
            continue;
        }

        if in_marker == true {
            str.push(ch);
            continue;
        }

        c += 1;
    }

    let result = c + markers.iter().fold(0, |a, b| a + b.len * b.n) as isize;
    result.solution()
}

pub fn part2() -> Option<Solution> {
    decompress(INPUT).solution()
}

fn decompress(str: &str) -> isize {
    let mut c: isize = 0;
    let mut in_marker = false;
    let mut marker_str = String::with_capacity(3);
    let mut skip_str = String::with_capacity(3);

    let mut marker = Marker { len: 0, n: 0 };
    
    for ch in str.chars() {
        if marker.len > 0 {
            skip_str.push(ch);
            marker.len -= 1;
            continue;
        }

        if skip_str.len() > 0 {
            c += decompress(&skip_str) * marker.n as isize;
            skip_str.clear();
        }

        if ch == '(' {
            in_marker = true;
            continue;
        }

        if ch == ')' {
            let split = marker_str.split('x').collect_vec();
            marker.len = split[0].parse().unwrap();
            marker.n = split[1].parse().unwrap();
            
            marker_str.clear();
            in_marker = false;
            continue;
        }

        if in_marker == true {
            marker_str.push(ch);
            continue;
        }

        c += 1;
    }

    if skip_str.len() > 0 {
        c += decompress(&skip_str) * marker.n as isize;
    }

    c
}

#[derive(Debug)]
struct Marker {
    len: usize,
    n: usize,
}