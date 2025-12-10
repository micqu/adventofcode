use itertools::Itertools;

use crate::utils::{
    Parsable,
    points::point2d::{self, Point2d},
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Movie Theater";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut v: Vec<Point2d> = Vec::new();
    for line in INPUT.lines() {
        let mut bytes = line.bytes();
        v.push(Point2d::parse(&mut bytes));
    }

    let mut m = 0;
    for i in 0..v.len() - 1 {
        for j in i + 1..v.len() {
            let d = v[i] - v[j];
            m = m.max((d.x.abs() + 1) * (d.y.abs() + 1));
        }
    }

    m.solution()
}

pub fn part2() -> Option<Solution> {
    let mut v: Vec<Point2d> = Vec::new();
    for line in INPUT.lines() {
        let mut bytes = line.bytes();
        v.push(Point2d::parse(&mut bytes));
    }

    let mut edges = Vec::new();
    let mut dx = Vec::new();

    for (a, b) in v.iter().cycle().take(v.len() + 1).tuple_windows() {
        let d = b - a;
        if d.x != 0 {
            dx.push((a, b, d.x > 0));
        }

        edges.push((a, b));
    }

    dx.sort_unstable_by_key(|a| a.0.y);

    let sizes = v
        .iter()
        .cycle()
        .take(v.len() + 1)
        .tuple_combinations()
        .map(|(a, b)| {
            let d = b - a;
            ((d.x.abs() + 1) * (d.y.abs() + 1), a, b)
        })
        .sorted_unstable_by(|a, b| b.0.cmp(&a.0))
        .collect_vec();

    for (area, a, b) in sizes {
        if a.x == b.x || a.y == b.y {
            continue;
        }

        let d = b - a;
        let mut ok = true;

        let (x_min, x_max) = (a.x.min(b.x) + 1, a.x.max(b.x) - 1);
        let (y_min, y_max) = (a.y.min(b.y) + 1, a.y.max(b.y) - 1);

        let inner = [
            (&Point2d::new(x_min, y_min), &Point2d::new(x_max, y_min)),
            (&Point2d::new(x_min, y_min), &Point2d::new(x_min, y_max)),
            (&Point2d::new(x_max, y_min), &Point2d::new(x_max, y_max)),
            (&Point2d::new(x_min, y_max), &Point2d::new(x_max, y_max)),
        ];

        let mid = Point2d::new((x_min + x_max) / 2, (y_min + y_max) / 2);

        for n in [Point2d::new(a.x + d.x, a.y), Point2d::new(a.x, a.y + d.y)] {
            if !inside(&n, &dx) {
                ok = false;
                break;
            }

            for &(e1, e2) in &edges {
                for l in inner {
                    if line_segment_intersects((&e1, &e2), l) {
                        ok = false;
                        break;
                    }
                }

                if !inside(&mid, &dx) {
                    ok = false;
                    break;
                }

                if !ok {
                    break;
                }
            }

            if !ok {
                break;
            }
        }

        if ok {
            return area.solution();
        }
    }

    None
}

fn inside(p: &Point2d, dx: &Vec<(&Point2d, &Point2d, bool)>) -> bool {
    for (a, b, d) in dx {
        if a.y > p.y {
            break;
        }

        if (p == *a) || (p == *b) {
            return true;
        }

        if a.x.min(b.x) <= p.x && p.x <= a.x.max(b.x) {
            if p.y == a.y {
                return true;
            }

            return *d;
        }
    }

    false
}

fn line_segment_intersects(a: (&Point2d, &Point2d), b: (&Point2d, &Point2d)) -> bool {
    orient(a.0, a.1, b.0).strict_mul(orient(a.0, a.1, b.1)) < 0
        && orient(b.0, b.1, a.0).strict_mul(orient(b.0, b.1, a.1)) < 0
}

fn orient(a: &Point2d, b: &Point2d, c: &Point2d) -> i128 {
    cross(&(b - a), &(c - a)) as i128
}

fn cross(a: &Point2d, b: &Point2d) -> isize {
    a.x * b.y - a.y * b.x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (4741451444 as isize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (1562459680 as isize).solution());
    }
}
