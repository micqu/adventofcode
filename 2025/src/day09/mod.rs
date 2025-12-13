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
    let mut v = parse();
    v.push(v[0]);

    let mut dx = Vec::new();
    let mut dy = Vec::new();

    for (a, b) in v.iter().tuple_windows() {
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
        .tuple_combinations()
        .filter_map(|(a, b)| {
            if a.x == b.x || a.y == b.y {
                None
            } else {
                Some(((a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1), a, b))
            }
        })
        .sorted_unstable_by(|a, b| b.0.cmp(&a.0));

    for (area, a, b) in sizes {
        let (x_min, x_max) = (a.x.min(b.x) + 1, a.x.max(b.x) - 1);
        let (y_min, y_max) = (a.y.min(b.y) + 1, a.y.max(b.y) - 1);

        if !valid_rectangle(x_min, x_max, y_min, y_max, &dx, &dy)
            || !inside(&Point2d::new(b.x, a.y), &dx)
            || !inside(&Point2d::new(a.x, b.y), &dx)
            || !inside(&Point2d::new(x_min, y_min), &dx)
        {
            continue;
        }

        return area.solution();
    }

    None
}

fn valid_rectangle(
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    dx: &Vec<(&Point2d, &Point2d, bool)>,
    dy: &Vec<(&Point2d, &Point2d)>,
) -> bool {
    let rect_horizontal = [
        (Point2d::new(x_min, y_min), Point2d::new(x_max, y_min)),
        (Point2d::new(x_min, y_max), Point2d::new(x_max, y_max)),
    ];

    for &e in dy {
        for (a, b) in rect_horizontal {
            if e.0.x < a.x || e.0.x > b.x {
                break;
            }

            if e.0.y.min(e.1.y) < a.y && a.y < e.0.y.max(e.1.y) {
                return false;
            }
        }
    }

    let rect_vertical = [
        (Point2d::new(x_min, y_min), Point2d::new(x_min, y_max)),
        (Point2d::new(x_max, y_min), Point2d::new(x_max, y_max)),
    ];

    for &e in dx {
        for (a, b) in rect_vertical {
            if e.0.y < a.y || e.0.y > b.y {
                break;
            }

            if e.0.x.min(e.1.x) < a.x && a.x < e.0.x.max(e.1.x) {
                return false;
            }
        }
    }

    true
}

fn inside(p: &Point2d, dx: &Vec<(&Point2d, &Point2d, bool)>) -> bool {
    for (a, b, d) in dx.iter().rev() {
        if a.x.min(b.x) <= p.x && p.x <= a.x.max(b.x) {
            if p.y == a.y {
                return true;
            }

            if a.y < p.y {
                return *d;
            }
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
