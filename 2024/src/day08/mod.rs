use std::collections::{HashMap, HashSet};

use crate::utils::{
    solution::{IntoSolution, Solution},
    vec2d::Vec2d,
};

pub const TITLE: &str = "Resonant Collinearity";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (lookup, w, h) = parse();
    let mut result = Vec2d::<bool>::from_vec_width(vec![false; w * h], w);

    for chs in lookup.into_iter().filter_map(|x| x) {
        for i in 0..chs.len() - 1 {
            let a = chs[i];

            for j in (i + 1)..chs.len() {
                let b = chs[j];

                let d = (b.0 - a.0, b.1 - a.1);
                let aa = (a.0 - d.0, a.1 - d.1);
                if let Some(p) = result.contains(&aa) {
                    result[p] = true;
                }

                let bb = (b.0 + d.0, b.1 + d.1);
                if let Some(p) = result.contains(&bb) {
                    result[p] = true;
                }
            }
        }
    }

    result.data.into_iter().filter(|x| *x).count().solution()
}

pub fn part2() -> Option<Solution> {
    let (lookup, w, h) = parse();
    let mut result = Vec2d::<bool>::from_vec_width(vec![false; w * h], w);

    for chs in lookup.into_iter().filter_map(|x| x) {
        for i in 0..chs.len() - 1 {
            let a = chs[i];
            result[(a.0 as usize, a.1 as usize)] = true;

            for j in (i + 1)..chs.len() {
                let b = chs[j];
                result[(b.0 as usize, b.1 as usize)] = true;

                let d = (b.0 - a.0, b.1 - a.1);
                let mut aa = (a.0 - d.0, a.1 - d.1);
                while let Some(p) = result.contains(&aa) {
                    result[p] = true;
                    aa = (aa.0 - d.0, aa.1 - d.1);
                }

                let mut bb = (b.0 + d.0, b.1 + d.1);
                while let Some(p) = result.contains(&bb) {
                    result[p] = true;
                    bb = (bb.0 + d.0, bb.1 + d.1);
                }
            }
        }
    }

    result.data.into_iter().filter(|x| *x).count().solution()
}

fn parse() -> (Vec<Option<Vec<(isize, isize)>>>, usize, usize) {
    let mut w: usize = 0;
    let mut h: usize = 0;
    let mut lookup: Vec<Option<Vec<(isize, isize)>>> = vec![None; 256];

    for line in INPUT.lines() {
        w = line.len();
        for (x, ch) in line.bytes().enumerate() {
            if ch != b'.' {
                if let Some(v) = &mut lookup[ch as usize] {
                    v.push((x as isize, h as isize));
                } else {
                    lookup[ch as usize] = Some(vec![(x as isize, h as isize)]);
                }
            }
        }
        h += 1;
    }

    (lookup, w, h)
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