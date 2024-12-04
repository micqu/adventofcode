use itertools::Itertools;

use crate::utils::{solution::{IntoSolution, Solution}, vec2d::Vec2d};

pub const TITLE: &str = "Ceres Search";
const INPUT: &'static str = include_str!("input.txt");

const DIR_EIGHT: [(isize, isize); 8] = [(1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)];
const DIR_DIAG: [(isize, isize); 4] = [(1, -1), (-1, -1), (-1, 1), (1, 1)];

pub fn part1() -> Option<Solution> {
    let m = parse();
    let xmas = "XMAS".bytes().collect_vec();
    let mut s = 0;
    for i in 0..m.width {
        for j in 0..m.height {
            if m[(i, j)] != xmas[0] {
                continue;
            }
            
            for (dx, dy) in DIR_EIGHT {
                let mut c = 0;
                for k in 1..4 {
                    if let Some((x, y)) = is_inside(&m, &(i as isize + dx * k, j as isize + dy * k)) {
                        if m[(x as usize, y as usize)] != xmas[k as usize] {
                            break;
                        }
    
                        c += 1;
                        if c == 3 {
                            s += 1;
                        }
                    }
                }
            }
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let m = parse();
    let mut s = 0;
    for i in 0..m.width {
        for j in 0..m.height {
            if m[(i, j)] != b'A' {
                continue;
            }

            let mut c = 0;
            for k in 0..2 {
                let (dx, dy) = DIR_DIAG[k];
                if let Some((x, y)) = is_inside(&m, &(i as isize + dx, j as isize + dy)) {
                    let p = m[(x, y)];
                    if p != b'S' && p != b'M' {
                        break;
                    }

                    let (dx2, dy2) = DIR_DIAG[k + 2];
                    if let Some((x2, y2)) = is_inside(&m, &(i as isize + dx2, j as isize + dy2)) {
                        let p2 = m[(x2, y2)];
                        if p2 != b'S' && p2 != b'M' {
                            break;
                        }

                        if p != p2 {
                            c += 1;
                            if c == 2 {
                                s += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    s.solution()
}

fn parse() -> Vec2d<u8> {
    let mut h = 0;
    let k = INPUT.lines().inspect(|_| h += 1).map(|x| x.bytes()).flatten().collect();
    Vec2d::from_vec(k, h)
}

fn is_inside(m: &Vec2d<u8>, p: &(isize, isize)) -> Option<(usize, usize)> {
    if p.0 < 0 || p.0 >= m.width as isize || p.1 < 0 || p.1 >= m.height as isize {
        return None;
    }

    Some((p.0 as usize, p.1 as usize))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::I32(a) => assert_eq!(a, 2646),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::I32(a) => assert_eq!(a, 2000),
            _ => panic!(),
        }
    }
}