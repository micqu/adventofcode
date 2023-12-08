use itertools::Itertools;

use crate::utils::{solution::{Solution, IntoSolution}, ADJ_EIGHT};

const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut s: u32 = 0;

    const H: usize = 140;
    const W: usize = 141;
    let map = INPUT.chars().collect_vec();

    for i in 0..H {
        let mut n = 0;
        let mut found_symbol = false;

        for j in 0..W - 1 {
            let ch = map[i * W + j];
            if let Some(digit) = ch.to_digit(10) {
                n = n * 10 + digit;

                if !found_symbol {
                    for adj in ADJ_EIGHT {
                        let nx = j as isize + adj.0;
                        let ny = i as isize + adj.1;
                        
                        if nx < 0 || nx >= W as isize - 1 || ny < 0 || ny >= H as isize {
                            continue;
                        }
                        
                        let ch = map[ny as usize * W + nx as usize];
                        if ch != '.' && ch.is_ascii_punctuation() {
                            found_symbol = true;
                            break;
                        }
                    }
                }
            } else {
                if found_symbol {
                    s += n;
                    found_symbol = false;
                }

                n = 0;
            }
        }

        if found_symbol {
            s += n;
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let mut s: u32 = 0;

    const H: usize = 140;
    const W: usize = 141;
    let map = INPUT.chars().collect_vec();

    for i in 0..H {
        for j in 0..W - 1 {
            if map[i * W + j] == '*' {
                s += calculate_gear_ratio(j, i, W, H, &map).unwrap_or(0);
            }
        }
    }

    s.solution()
}

fn calculate_gear_ratio(x: usize, y: usize, w: usize, h: usize, map: &Vec<char>) -> Option<u32> {
    let mut c = 0;
    let mut s = 1;

    for adj in ADJ_EIGHT {
        let nx = x as isize + adj.0;
        let ny = y as isize + adj.1;

        if nx < 0 || nx >= w as isize || ny < 0 || ny >= h as isize {
            continue;
        }

        let ny = ny as usize;
        let nx = nx as usize;

        if nx > x.saturating_sub(1) && map[ny * w + nx - 1].is_ascii_digit() {
            continue;
        }

        if map[ny * w + nx].is_ascii_digit() {
            if c == 2 {
                return None;
            }

            s *= expand(nx, ny, w, &map);
            c += 1;
        }
    }

    (c == 2).then(|| s)
}

fn expand(x: usize, y: usize, w: usize, map: &Vec<char>) -> u32 {
    let mut start = x;
    while start > 0 {
        if map[y * w + start - 1].is_ascii_digit() {
            start -= 1;
        } else {
            break;
        }
    }
    
    let mut n = 0;
    for i in start..w - 1 {
        if let Some(digit) = map[y * w + i].to_digit(10) {
            n = n * 10 + digit;
        } else {
            break;
        }
    }
    
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 526404),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 84399773),
            _ => panic!(),
        }
    }
}