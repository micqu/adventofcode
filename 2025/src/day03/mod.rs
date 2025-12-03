use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Lobby";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    INPUT
        .lines()
        .filter_map(|line| solve(line, 0, 2))
        .sum::<usize>()
        .solution()
}

pub fn part2() -> Option<Solution> {
    INPUT
        .lines()
        .filter_map(|line| solve(line, 0, 12))
        .sum::<usize>()
        .solution()
}

fn solve(line: &str, pos: usize, rem: usize) -> Option<usize> {
    if rem == 0 {
        return None;
    }

    let mut bytes = line.bytes().skip(pos);
    let mut max = 0;
    let mut next = 0;

    for i in pos..line.len() - rem + 1 {
        let a = bytes.next().unwrap();
        if a > max {
            max = a;
            next = i;
        }
    }

    if let Some(s) = solve(line, next + 1, rem - 1) {
        let l = s.checked_ilog10().unwrap_or(0) + 1;
        Some((max - b'0') as usize * 10_usize.pow(l) + s)
    } else {
        Some((max - b'0') as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (17412 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (172681562473501 as usize).solution());
    }
}
