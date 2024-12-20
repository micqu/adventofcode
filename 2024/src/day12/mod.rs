use crate::utils::{
    grid::grid::Grid, points::point2d::Point2d, solution::{IntoSolution, Solution}
};

pub const TITLE: &str = "Garden Groups";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let map = parse();
    let mut seen = Grid::from_vec_width(vec![false; map.data.len()], map.width);
    map.positions_point2d()
        .map(|p| {
            if !seen[p] {
                let r = fill(&p, &map, &mut seen);
                return r.0 * r.1;
            }
            0
        })
        .sum::<usize>()
        .solution()
}

pub fn part2() -> Option<Solution> {
    let map = parse();
    let mut seen = Grid::from_vec_width(vec![false; map.data.len()], map.width);
    map.positions_point2d()
        .map(|p| {
            if !seen[p] {
                let r = fill2(&p, &map, &mut seen);
                return r.0 * r.1;
            }
            0
        })
        .sum::<usize>()
        .solution()
}

fn fill(p: &Point2d, map: &Grid<u8>, seen: &mut Grid<bool>) -> (usize, usize) {
    seen[p] = true;
    let mut perimeter = 4;
    let mut area = 1;
    let c = map[p];
    for (n, _) in map.four_connected_point2d(p) {
        if map[n] == c {
            perimeter -= 1;
            if !seen[n] {
                let r = fill(&n, map, seen);
                perimeter += r.0;
                area += r.1;
            }
        }
    }
    (perimeter, area)
}

fn fill2(p: &Point2d, map: &Grid<u8>, seen: &mut Grid<bool>) -> (usize, usize) {
    seen[p] = true;
    let mut area = 1;
    let mut corners = 0;
    let c = map[p];
    let mut ds: usize = 0b1111;
    for (n, d) in map.four_connected_point2d(p) {
        if map[n] == c {
            ds &= !(1 << d);
            if !seen[n] {
                let r = fill2(&n, map, seen);
                corners += r.0;
                area += r.1;
            }
        }
    }

    if ds & 0b0011 == 0b0011 {
        corners += 1;
    }

    if ds & 0b0110 == 0b0110 {
        corners += 1;
    }

    if ds & 0b1100 == 0b1100 {
        corners += 1;
    }

    if ds & 0b1001 == 0b1001 {
        corners += 1;
    }

    let mut diag: usize = 0b1111;
    for (n, d) in map.diagonals_point2d(p) {
        if map[n] == c {
            diag &= !(1 << d);
        }
    }

    if ds & 0b1100 == 0b1000 && diag & 0b0100 == 0 {
        corners += 1;
    }

    if ds & 0b1001 == 0b1000 && diag & 0b1000 == 0 {
        corners += 1;
    }

    if ds & 0b1001 == 0b0001 && diag & 0b1000 == 0 {
        corners += 1;
    }

    if ds & 0b1100 == 0b0100 && diag & 0b0100 == 0 {
        corners += 1;
    }

    (corners, area)
}

fn parse() -> Grid<u8> {
    let mut h: usize = 0;
    let k = INPUT
        .lines()
        .inspect(|_| h += 1)
        .map(|x| x.bytes())
        .flatten()
        .collect();

    Grid::from_vec_height(k, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (1421958 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (885394 as usize).solution());
    }
}
