use num::integer::Roots;

use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Gift Shop";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    None
}

pub fn part2() -> Option<Solution> {
    let mut ids = 0;
    let ranges = parse();

    for Range { from, to } in ranges {
        for n in from..=to {
            let len = length(n);

            for i in 1..=len / 2 {
                if len % i == 0 {
                    let o = 10_u32.pow(i - 1);
                    let k = mirror(o, i, len / i);
                    if n % (k / o) as usize == 0 {
                        ids += n;
                        break;
                    }
                }
            }
        }
    }

    ids.solution()
}

fn length(a: usize) -> u32 {
    a.checked_ilog10().unwrap_or(0) + 1
}

fn mirror(a: u32, len: u32, n: u32) -> u32 {
    let mut r = 0;
    for _ in 0..n {
        r = r * 10_u32.pow(len) + a;
    }
    r
}

fn parse() -> Vec<Range> {
    let mut ranges = Vec::new();
    let mut bytes = INPUT.bytes();
    while let Some(a) = Parsable::<usize>::next_number(&mut bytes) {
        let b = bytes.next_number().unwrap();
        ranges.push(Range { from: a, to: b });
    }

    ranges
}

#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (5398419778 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (15704845910 as usize).solution());
    }
}
