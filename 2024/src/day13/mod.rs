use crate::utils::{
    point::Point,
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "?!";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let cs = parse();
    let mut s = 0;
    for c in cs {
        let d = det(c.a.x, c.b.y, c.b.x, c.a.y);
        let dx = det(c.prize.x, c.b.y, c.b.x, c.prize.y);
        let dy = det(c.a.x, c.prize.y, c.prize.x, c.a.y);
        if dx % d == 0 && dy % d == 0 {
            let x = dx / d;
            let y = dy / d;
            if x <= 100 && y <= 100 {
                s += (3 * x + y) as usize;
            }
        }

    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let cs = parse();
    let mut s = 0;
    for mut c in cs {
        c.prize.x += 10000000000000;
        c.prize.y += 10000000000000;
        let d = det(c.a.x, c.b.y, c.b.x, c.a.y);
        let dx = det(c.prize.x, c.b.y, c.b.x, c.prize.y);
        let dy = det(c.a.x, c.prize.y, c.prize.x, c.a.y);
        if dx % d == 0 && dy % d == 0 {
            let x = dx / d;
            let y = dy / d;
            s += (3 * x + y) as usize;
        }
    }

    s.solution()
}

fn det(a: isize, b: isize, c: isize, d: isize) -> isize {
    a * b - c * d
}

fn parse() -> Vec<ClawMachine> {
    let mut cms = Vec::new();
    let mut numbers = INPUT.bytes();
    while let Some(n) = numbers.next_number() {
        cms.push(ClawMachine {
            a: Point::new(n, numbers.next_number().unwrap()),
            b: Point::new(
                numbers.next_number().unwrap(),
                numbers.next_number().unwrap(),
            ),
            prize: Point::new(
                numbers.next_number().unwrap(),
                numbers.next_number().unwrap(),
            ),
        });
    }
    cms
}

#[derive(Debug)]
struct ClawMachine {
    a: Point,
    b: Point,
    prize: Point,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 34787),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 85644161121698),
            _ => panic!(),
        }
    }
}
