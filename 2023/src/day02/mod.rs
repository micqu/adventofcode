use crate::utils::solution::{Solution, IntoSolution};

const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    INPUT.lines().enumerate().map(|(id, line)| {
        let mut splits = line.split(&[',', ';', ':', ' ']).skip(2);
        while let Some(split) = splits.next() {
            if let Ok(n) = split.parse::<usize>() {
                let b = splits.next().unwrap();
                match b.chars().nth(0).unwrap() {
                    'r' if n > 12 => return 0,
                    'g' if n > 13 => return 0,
                    'b' if n > 14 => return 0,
                    _ => { },
                }
            }
        }
        
        id + 1
    }).sum::<usize>().solution()
}

pub fn part2() -> Option<Solution> {
    INPUT.lines().map(|line| {
        let (mut r, mut g, mut b) = (0, 0, 0);
        let mut splits = line.split(&[',', ';', ':', ' ']).skip(2);
        while let Some(s) = splits.next() {
            if let Ok(n) = s.parse::<usize>() {
                let c = splits.next().unwrap();
                match c.chars().nth(0).unwrap() {
                    'r' => r = r.max(n),
                    'g' => g = g.max(n),
                    'b' => b = b.max(n),
                    _ => panic!(),
                }
            }
        }

        r * g * b
    }).sum::<usize>().solution()
}