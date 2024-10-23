use itertools::Itertools;

use crate::utils::{
    solution::{IntoSolution, Solution}, vec3::Vec3, Parsable
};

pub const TITLE: &str = "Never Tell Me The Odds";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let hailstones = parse();
    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;
    let mut s: usize = 0;
    for (a, b) in hailstones.iter().tuple_combinations() {
        let w = a.d.cross2d(&b.d);
        if w.abs() <= f64::EPSILON {
            continue;
        }

        let d = b.p.sub(&a.p);
        let t = d.cross2d(&b.d) / w;
        let u = d.cross2d(&a.d) / w;
        if t > f64::EPSILON && u > f64::EPSILON {
            let k = b.p.add(&b.d.mul_n(u));
            if k.x >= MIN && k.x <= MAX && k.y >= MIN && k.y <= MAX {
                s += 1;
            }
        }
    }
    s.solution()
}

pub fn part2() -> Option<Solution> {
    let hailstones = parse();
    let r = &hailstones[0];
    let a = &hailstones[1].sub(r);
    let n = a.p.cross3d(&a.d);
    
    let b = &hailstones[2].sub(r);
    let bt = n.dot3d(&a.p.sub(&b.p)) / n.dot3d(&b.d);
    let bintersect = b.p.add(&b.d.mul_n(bt));
    
    let c = &hailstones[3].sub(r);
    let ct = n.dot3d(&a.p.sub(&c.p)) / n.dot3d(&c.d);
    let cintersect = c.p.add(&c.d.mul_n(ct));

    let dr = bintersect.sub(&cintersect).div_n(bt - ct);
    let pr = bintersect.sub(&dr.mul_n(bt));
    let p = pr.add(&r.p);

    (p.x.round() + p.y.round() + p.z.round()).solution()
}

fn parse() -> Vec<Hailstone> {
    let mut lines = Vec::new();
    let mut bytes = INPUT.bytes();
    while let Some(a) = bytes.next_number() {
        let p = Vec3 {
            x: a,
            y: bytes.next_number().unwrap(),
            z: bytes.next_number().unwrap(),
        };

        let d = Vec3 {
            x: bytes.next_number().unwrap(),
            y: bytes.next_number().unwrap(),
            z: bytes.next_number().unwrap(),
        };

        lines.push(Hailstone { p, d });
    }
    lines
}

#[derive(Debug)]
struct Hailstone {
    p: Vec3,
    d: Vec3,
}

impl Hailstone {
    fn sub(&self, other: &Self) -> Self {
        Self {
            p: self.p.sub(&other.p),
            d: self.d.sub(&other.d),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 16502),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::F64(a) => assert_eq!(a, 673641951253289.0),
            _ => panic!(),
        }
    }
}