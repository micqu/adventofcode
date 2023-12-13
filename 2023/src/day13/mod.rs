use itertools::Itertools;

use crate::utils::solution::{IntoSolution, Solution};

pub const TITLE: &str = "Point of Incidence";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    parse()
        .iter()
        .map(|map| {
            let h = map.len();
            let w = map[0].len();

            let mut x_cmax = 0;
            let mut x = 0;
            for i in 0..w {
                let (c, _) = xsplit(i, &map, false);
                if c > x_cmax {
                    x_cmax = c;
                    x = i;
                }
            }

            let mut y_cmax = 0;
            let mut y = 0;
            for i in 0..h {
                let (c, _) = ysplit(i, &map, false);
                if c > y_cmax {
                    y_cmax = c;
                    y = i;
                }
            }

            if x_cmax > y_cmax { x } else { y * 100 }
        })
        .sum::<usize>()
        .solution()
}

pub fn part2() -> Option<Solution> {
    parse()
        .iter()
        .map(|map| {
            let h = map.len();
            let w = map[0].len();
            
            let mut x_cmax = 0;
            let mut x = 0;
            for i in 0..w {
                let (c, has_smudge) = xsplit(i, &map, true);
                if c > x_cmax && has_smudge {
                    x_cmax = c;
                    x = i;
                }
            }

            let mut y_cmax = 0;
            let mut y = 0;
            for i in 0..h {
                let (c, has_smudge) = ysplit(i, &map, true);
                if c > y_cmax && has_smudge {
                    y_cmax = c;
                    y = i;
                }
            }

            if x_cmax > y_cmax { x } else { y * 100 }
        })
        .sum::<usize>()
        .solution()
}

type Map = Vec<Vec<u8>>;

fn parse() -> Vec<Map> {
    let mut maps = vec![Map::new()];
    for line in INPUT.lines() {
        if line.is_empty() {
            maps.push(Map::new());
            continue;
        }

        maps.last_mut().unwrap().push(line.bytes().collect_vec());
    }

    maps
}

fn xsplit(xsplit: usize, map: &Map, mut smudge: bool) -> (u32, bool) {
    let h = map.len();
    let w = map[0].len();
    let mut s = 0;
    for i in 0..h {
        for j in xsplit..w {
            let mj = xsplit as isize - (j - xsplit) as isize - 1;

            if mj < 0 {
                continue;
            }

            if map[i][j] == map[i][mj as usize] {
                s += 1;
            } else {
                if !smudge {
                    return (0, true);
                }

                smudge = false;
            }
        }
    }

    (s, !smudge)
}

fn ysplit(ysplit: usize, map: &Map, mut smudge: bool) -> (u32, bool) {
    let h = map.len();
    let w = map[0].len();
    let mut s = 0;
    for i in ysplit..h {
        let mi = ysplit as isize - (i - ysplit) as isize - 1;

        if mi < 0 {
            continue;
        }

        for j in 0..w {
            if map[i][j] == map[mi as usize][j] {
                s += 1;
            } else {
                if !smudge {
                    return (0, true);
                }

                smudge = false;
            }
        }
    }

    (s, !smudge)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 29130),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 33438),
            _ => panic!(),
        }
    }
}
