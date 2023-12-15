use crate::utils::solution::{IntoSolution, Solution};

pub const TITLE: &str = "Lens Library";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    INPUT.split(',').map(hash).sum::<u32>().solution()
}

pub fn part2() -> Option<Solution> {
    let mut boxes = vec![Vec::<Lens>::new(); 256];
    for step in INPUT.split(',') {
        let (op_idx, op) = step.match_indices(|c| c == '-' || c == '=').next().unwrap();
        let (label, focal_length) = (step.get(0..op_idx).unwrap(), step.get(op_idx + 1..).unwrap());
        let b = hash(label) as usize;
        match op {
            "-" => {
                if let Some(p) = boxes[b].iter().position(|x| x.label == label) {
                    boxes[b].remove(p);
                }
            }
            "=" => {
                let focal_length = focal_length.parse::<u32>().unwrap();

                if let Some(p) = boxes[b].iter().position(|x| x.label == label) {
                    let lens = &mut boxes[b][p];
                    lens.focal_length = focal_length;
                    lens.label = label;
                } else {
                    boxes[b].push(Lens { focal_length, label });
                }
            }
            _ => {}
        }
    }

    let mut s = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, l) in b.iter().enumerate() {
            s += (i + 1) * (j + 1) * l.focal_length as usize;
        }
    }
    s.solution()
}

fn hash(a: &str) -> u32 {
    let mut s = 0;
    for ch in a.bytes() {
        s += ch as u32;
        s *= 17;
        s %= 256;
    }
    s
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lens {
    focal_length: u32,
    label: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 514394),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 236358),
            _ => panic!(),
        }
    }
}
