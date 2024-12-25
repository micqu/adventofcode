use std::io::Read;

use crate::utils::{grid::grid::Grid, solution::{IntoSolution, Solution}};

pub const TITLE: &str = "Code Chronicle";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (keys, locks) = parse();
    
    let mut s = 0;
    for lock in &locks {
        for k in &keys {
            let mut ok = true;
            for i in 0..lock.len() {
                if k[i] + lock[i] > 7 {
                    ok = false;
                    break;
                }
            }
            if ok {
                s += 1;
            }
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    None
}

fn parse() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut map = Grid::<u8>::new(Vec::new(), 5, 7);
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut lines = INPUT.lines();
    
    loop {
        if let Some(line) = lines.next() {
            map.data.extend(line.bytes());
            for _ in 0..map.height - 1 {
                map.data.extend(lines.next().unwrap().bytes());
            }

            let is_lock = map.data.iter().take(map.width).all(|x| *x == b'#');
            if is_lock {
                let mut lock = Vec::new();
                for i in 0..map.width {
                    for j in 0..map.height {
                        if map[(i, j)] != b'#' {
                            lock.push(j);
                            break;
                        }
                    }
                }
                locks.push(lock);
            } else {
                let mut key = Vec::new();
                for i in 0..map.width {
                    for j in 0..map.height {
                        if map[(i, j)] != b'.' {
                            key.push(map.height - j);
                            break;
                        }
                    }
                }
                keys.push(key);
            }
            map.data.clear();
            
            lines.next();
        } else {
            break;
        }
    }

    (keys, locks)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        assert_eq!(super::part1(), (3242 as i32).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), None);
    }
}