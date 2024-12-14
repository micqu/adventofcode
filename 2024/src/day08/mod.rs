use crate::utils::{
    solution::{IntoSolution, Solution},
    vec2d::Vec2d,
};

pub const TITLE: &str = "Resonant Collinearity";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (antennas, w, h) = parse();
    let mut antinodes = Vec2d::<bool>::from_vec_width(vec![false; w * h], w);
    let mut s = 0;
    for chs in antennas.into_iter().filter_map(|x| x) {
        for i in 0..chs.len() - 1 {
            let a = chs[i];

            for j in (i + 1)..chs.len() {
                let b = chs[j];

                let d = (b.0 - a.0, b.1 - a.1);
                let aa = (a.0 - d.0, a.1 - d.1);
                if let Some(p) = antinodes.contains(&aa) {
                    if !antinodes[p] {
                        s += 1;
                        antinodes[p] = true;
                    }
                }

                let bb = (b.0 + d.0, b.1 + d.1);
                if let Some(p) = antinodes.contains(&bb) {
                    if !antinodes[p] {
                        s += 1;
                        antinodes[p] = true;
                    }
                }
            }
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let (antennas, w, h) = parse();
    let mut antinodes = Vec2d::<bool>::from_vec_width(vec![false; w * h], w);
    let mut s = 0;
    for chs in antennas.into_iter().filter_map(|x| x) {
        for i in 0..chs.len() - 1 {
            let a = chs[i];
            antinodes[(a.0 as usize, a.1 as usize)] = true;

            for j in (i + 1)..chs.len() {
                let b = chs[j];
                antinodes[(b.0 as usize, b.1 as usize)] = true;

                let d = (b.0 - a.0, b.1 - a.1);
                let mut aa = (a.0 - d.0, a.1 - d.1);
                while let Some(p) = antinodes.contains(&aa) {
                    if !antinodes[p] {
                        s += 1;
                        antinodes[p] = true;
                    }
                    aa = (aa.0 - d.0, aa.1 - d.1);
                }

                let mut bb = (b.0 + d.0, b.1 + d.1);
                while let Some(p) = antinodes.contains(&bb) {
                    if !antinodes[p] {
                        s += 1;
                        antinodes[p] = true;
                    }
                    bb = (bb.0 + d.0, bb.1 + d.1);
                }
            }
        }
    }

    s.solution()
}

fn parse() -> (Vec<Option<Vec<(isize, isize)>>>, usize, usize) {
    let mut w: usize = 0;
    let mut h: usize = 0;
    let mut antennas: Vec<Option<Vec<(isize, isize)>>> = vec![None; 256];

    for line in INPUT.lines() {
        w = line.len();
        for (x, ch) in line.bytes().enumerate() {
            if ch != b'.' {
                if let Some(v) = &mut antennas[ch as usize] {
                    v.push((x as isize, h as isize));
                } else {
                    antennas[ch as usize] = Some(vec![(x as isize, h as isize)]);
                }
            }
        }
        h += 1;
    }

    (antennas, w, h)
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