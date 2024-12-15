use crate::utils::{
    points::point2d::Point2d,
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Claw Contraption";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let cs = parse();
    let mut s = 0;
    for c in cs {
        let d = c.a.determinant(&c.b);
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
        let d = c.a.determinant(&c.b);
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
            a: Point2d::new(n, numbers.next_number().unwrap()),
            b: Point2d::parse(&mut numbers),
            prize: Point2d::parse(&mut numbers),
        });
    }
    cms
}

#[derive(Debug)]
struct ClawMachine {
    a: Point2d,
    b: Point2d,
    prize: Point2d,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (34787 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (85644161121698 as usize).solution());
    }
}
