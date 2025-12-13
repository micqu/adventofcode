use crate::utils::{
    NextNumbers, Parsable,
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Christmas Tree Farm";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (shapes, regions) = parse();
    let mut s = 0;
    for r in regions {
        let a = r.width * r.height;
        let mut t = 0;
        for (i, c) in r.counts.iter().enumerate() {
            t += shapes[i].1 * c;
        }

        if t <= a {
            s += 1;
        }
    }
    s.solution()
}

pub fn part2() -> Option<Solution> {
    None
}

fn parse() -> (Vec<(u16, usize)>, Vec<Region>) {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();
    let mut sw = false;
    let mut shape: u16 = 0;

    for line in INPUT.lines() {
        if line.is_empty() {
            if shape != 0 {
                shapes.push((shape, shape.count_ones() as usize));
                shape = 0;
            }
            continue;
        }

        if line.len() > 3 {
            sw = true;
        }

        let mut bytes = line.bytes();
        if sw {
            regions.push(Region {
                width: bytes.next_number().unwrap(),
                height: bytes.next_number().unwrap(),
                counts: bytes.next_numbers(),
            });
        } else {
            if line.len() == 3 {
                for _ in 0..3 {
                    match bytes.next().unwrap() {
                        b'#' => shape = (shape << 1) | 1,
                        b'.' => shape <<= 1,
                        _ => panic!(),
                    }
                }
            }
        }
    }

    (shapes, regions)
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (2196996 as u32).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (23655822 as u32).solution());
    }
}
