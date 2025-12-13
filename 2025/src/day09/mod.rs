use itertools::Itertools;

use crate::utils::{
    Parsable,
    points::point2d::{self, Point2d},
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Movie Theater";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    parse()
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .unwrap()
        .solution()
}

pub fn part2() -> Option<Solution> {
    let v = parse();
    let mut dx = Vec::new();
    let mut dy = Vec::new();

    for (a, b) in v.iter().cycle().take(v.len() + 1).tuple_windows() {
        if b.x - a.x != 0 {
            dx.push((a, b, b.x - a.x > 0));
        } else {
            dy.push((a, b));
        }
    }

    dx.sort_unstable_by_key(|a| a.0.y);
    dy.sort_unstable_by_key(|a| a.0.x);

    let sizes = v
        .iter()
        .cycle()
        .take(v.len() + 1)
        .tuple_combinations()
        .map(|(a, b)| ((a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1), a, b))
        .sorted_unstable_by(|a, b| b.0.cmp(&a.0))
        .collect_vec();

    for (area, a, b) in sizes {
        if a.x == b.x || a.y == b.y {
            continue;
        }

        let (x_min, x_max) = (a.x.min(b.x) + 1, a.x.max(b.x) - 1);
        let (y_min, y_max) = (a.y.min(b.y) + 1, a.y.max(b.y) - 1);

        let inner = [
            (&Point2d::new(x_min, y_min), &Point2d::new(x_max, y_min)),
            (&Point2d::new(x_min, y_max), &Point2d::new(x_max, y_max)),
            (&Point2d::new(x_min, y_min), &Point2d::new(x_min, y_max)),
            (&Point2d::new(x_max, y_min), &Point2d::new(x_max, y_max)),
        ];

        let mid = Point2d::new((x_min + x_max) / 2, (y_min + y_max) / 2);

        let d = b - a;
        if [Point2d::new(a.x + d.x, a.y), Point2d::new(a.x, a.y + d.y)]
            .iter()
            .all(|n| valid_rectangle(&n, &mid, inner, &dx, &dy))
        {
            return area.solution();
        }
    }

    None
}

fn valid_rectangle(
    n: &Point2d,
    mid: &Point2d,
    inner_rect: [(&Point2d, &Point2d); 4],
    dx: &Vec<(&Point2d, &Point2d, bool)>,
    dy: &Vec<(&Point2d, &Point2d)>,
) -> bool {
    for &e in dy {
        for &line in inner_rect.iter().take(2) {
            if e.0.x < line.0.x || e.0.x > line.1.x {
                break;
            }

            if e.0.y.min(e.1.y) < line.0.y && line.0.y < e.0.y.max(e.1.y) {
                return false;
            }
        }
    }

    for &e in dx {
        for &line in inner_rect.iter().skip(2) {
            if e.0.y < line.0.y || e.0.y > line.1.y {
                break;
            }

            if e.0.x.min(e.1.x) < line.0.x && line.0.x < e.0.x.max(e.1.x) {
                return false;
            }
        }
    }

    if !inside(&mid, &dx) || !inside(&n, &dx) {
        return false;
    }

    true
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

fn parse() -> Vec<Point2d> {
    let mut v = Vec::new();
    for line in INPUT.lines() {
        v.push(Point2d::parse(&mut line.bytes()));
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (4741451444 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (1562459680 as usize).solution());
    }
}
