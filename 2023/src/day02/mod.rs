use crate::utils::solution::Solution;

const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let s: usize = INPUT.lines().enumerate().map(|(id, line)| {
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
    }).sum();
    
    Some(s.into())
}

pub fn part2() -> Option<Solution> {
    let s: usize = INPUT.lines().map(|line| {
        let (mut sr, mut sg, mut sb) = (0, 0, 0);
        let mut splits = line.split(&[',', ';', ':', ' ']).skip(2);
        while let Some(s) = splits.next() {
            if let Ok(n) = s.parse::<usize>() {
                let b = splits.next().unwrap();
                match b.chars().nth(0).unwrap() {
                    'r' => sr = sr.max(n),
                    'g' => sg = sg.max(n),
                    'b' => sb = sb.max(n),
                    _ => panic!(),
                }
            }
        }

        sr * sg * sb
    }).sum();

    Some(s.into())
}