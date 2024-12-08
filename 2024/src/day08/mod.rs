use std::collections::{HashMap, HashSet};

use crate::utils::{
    solution::{IntoSolution, Solution},
    vec2d::Vec2d,
};

pub const TITLE: &str = "Resonant Collinearity";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (map, lookup) = parse();
    let mut result = HashSet::<(isize, isize)>::new();

    for (_, chs) in lookup {
        for i in 0..chs.len() - 1 {
            let a = chs[i];

            for j in (i + 1)..chs.len() {
                let b = chs[j];

                let d = (b.0 - a.0, b.1 - a.1);
                let aa = (a.0 - d.0, a.1 - d.1);
                if let Some(_) = map.contains(&aa) {
                    result.insert(aa);
                }
                
                let bb = (b.0 + d.0, b.1 + d.1);
                if let Some(_) = map.contains(&bb) {
                    result.insert(bb);
                }
            }
        }
    }

    result.len().solution()
}

pub fn part2() -> Option<Solution> {
    let (map, lookup) = parse();
    let mut result = HashSet::<(isize, isize)>::new();

    for (_, chs) in lookup {
        for i in 0..chs.len() - 1 {
            let a = chs[i];
            result.insert(a);

            for j in (i + 1)..chs.len() {
                let b = chs[j];
                result.insert(b);

                let d = (b.0 - a.0, b.1 - a.1);
                let mut aa = (a.0 - d.0, a.1 - d.1);
                while let Some(_) = map.contains(&aa) {
                    result.insert(aa);
                    aa = (aa.0 - d.0, aa.1 - d.1);
                }
                
                let mut bb = (b.0 + d.0, b.1 + d.1);
                while let Some(_) = map.contains(&bb) {
                    result.insert(bb);
                    bb = (bb.0 + d.0, bb.1 + d.1);
                }
            }
        }
    }

    result.len().solution()
}

fn parse() -> (Vec2d<u8>, HashMap<u8, Vec<(isize, isize)>>) {
    let mut h: usize = 0;
    let mut lookup = HashMap::<u8, Vec<(isize, isize)>>::new();
    let k = INPUT
        .lines()
        .map(|x| {
            for (i, ch) in x.bytes().enumerate() {
                if ch != b'.' {
                    lookup
                        .entry(ch)
                        .or_insert(Vec::new())
                        .push((i as isize, h as isize));
                }
            }
            h += 1;
            x.bytes()
        })
        .flatten()
        .collect();

    (Vec2d::from_vec_height(k, h), lookup)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 327),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 1233),
            _ => panic!(),
        }
    }
}
