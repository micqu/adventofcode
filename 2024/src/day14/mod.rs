use num::Integer;

use crate::utils::{
    point2d::Point2d,
    solution::{IntoSolution, Solution},
    vec2::Vec2,
    vec2d::Vec2d,
    Parsable,
};

pub const TITLE: &str = "Restroom Redoubt";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let robots = parse();
    let (w, h): (isize, isize) = (101, 103);
    let mut quadrant = [0; 4];
    for r in robots {
        let mut t: Point2d = r.pos + r.vel * 100;
        t.x = t.x.rem_euclid(w);
        t.y = t.y.rem_euclid(h);

        if t.x == w / 2 || t.y == h / 2 {
            continue;
        }

        let d = if t.x > w / 2 { 0b10 } else { 0 } | if t.y > h / 2 { 1 } else { 0 };
        quadrant[d] += 1;
    }

    quadrant.iter().product::<usize>().solution()
}

pub fn part2() -> Option<Solution> {
    let mut robots = parse();
    let (w, h): (isize, isize) = (101, 103);
    let mut s = usize::MAX;
    let mut id = -1;
    for i in 0..w * h {
        for r in &mut robots {
            r.pos += r.vel;
            r.pos.x = r.pos.x.rem_euclid(w);
            r.pos.y = r.pos.y.rem_euclid(h);
        }

        let v = variance(&robots);
        if v < s {
            s = v;
            id = i + 1;
        }
    }
    id.solution()
}

fn variance(robots: &Vec<Robot>) -> usize {
    let c = centroid(robots);
    let mut s = 0;
    for r in robots {
        s += r.pos.x.abs_diff(c.x) * r.pos.y.abs_diff(c.y);
    }
    s
}

fn centroid(robots: &Vec<Robot>) -> Point2d {
    robots.iter().map(|x| x.pos).sum::<Point2d>() / robots.len() as isize
}

fn parse() -> Vec<Robot> {
    INPUT
        .lines()
        .map(|line| {
            let mut line = line.bytes();
            let p = Point2d::parse(&mut line);
            let v = Point2d::parse(&mut line);
            Robot { pos: p, vel: v }
        })
        .collect()
}

struct Robot {
    pos: Point2d,
    vel: Point2d,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (216772608 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (6888 as isize).solution());
    }
}
