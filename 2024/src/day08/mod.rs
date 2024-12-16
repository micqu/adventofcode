use crate::utils::{
    solution::{IntoSolution, Solution},
    grid::grid::Grid,
};

pub const TITLE: &str = "Resonant Collinearity";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (antennas, w, h) = parse();
    let mut antinodes = Grid::<bool>::from_vec_width(vec![false; w * h], w);
    let mut s = 0;
    for chs in antennas.into_iter().filter_map(|x| x) {
        for i in 0..chs.len() - 1 {
            let a = chs[i];

            for j in (i + 1)..chs.len() {
                let b = chs[j];

                let d = (b.0 - a.0, b.1 - a.1);
                let aa = (a.0 - d.0, a.1 - d.1);
                if antinodes.contains_point(&aa) {
                    if !antinodes[aa] {
                        antinodes[aa] = true;
                        s += 1;
                    }
                }

                let bb = (b.0 + d.0, b.1 + d.1);
                if antinodes.contains_point(&bb) {
                    if !antinodes[bb] {
                        antinodes[bb] = true;
                        s += 1;
                    }
                }
            }
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let (antennas, w, h) = parse();
    let mut antinodes = Grid::<bool>::from_vec_width(vec![false; w * h], w);
    let mut s = 0;
    for chs in antennas.into_iter().filter_map(|x| x) {
        for i in 0..chs.len() - 1 {
            let a = chs[i];

            for j in (i + 1)..chs.len() {
                let b = chs[j];

                let d = (b.0 - a.0, b.1 - a.1);
                let mut aa = a;
                while antinodes.contains_point(&aa) {
                    if !antinodes[aa] {
                        antinodes[aa] = true;
                        s += 1;
                    }
                    aa = (aa.0 - d.0, aa.1 - d.1);
                }

                let mut bb = b;
                while antinodes.contains_point(&bb) {
                    if !antinodes[bb] {
                        antinodes[bb] = true;
                        s += 1;
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
        assert_eq!(super::part1(), (327 as i32).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (1233 as i32).solution());
    }
}