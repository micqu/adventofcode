use crate::utils::{
    solution::{IntoSolution, Solution},
    vec2d::Vec2d,
};

pub const TITLE: &str = "Hoof It";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let map = parse();
    let mut seen = Vec::<usize>::new();
    let mut s = 0;
    for i in 0..map.width {
        for j in 0..map.height {
            if map[(i, j)] == 0 {
                s += solve((i, j), 0, &map, &mut seen);
                seen.clear();
            }
        }
    }
    s.solution()
}

pub fn part2() -> Option<Solution> {
    let map = parse();
    let mut s = 0;
    for i in 0..map.width {
        for j in 0..map.height {
            if map[(i, j)] == 0 {
                s += solve2((i, j), 0, &map);
            }
        }
    }
    s.solution()
}

fn solve(pos: (usize, usize), i: u8, map: &Vec2d<u8>, seen: &mut Vec<usize>) -> usize {
    if i == 9 && !seen.contains(&(pos.0 * map.width + pos.1)) {
        seen.push(pos.0 * map.width + pos.1);
        return 1;
    }

    let mut s = 0;
    for (x, y, _) in map.four_connected_point(pos) {
        if map[(x, y)] == i + 1 {
            s += solve((x, y), i + 1, map, seen);
        }
    }
    s
}

fn solve2(pos: (usize, usize), i: u8, map: &Vec2d<u8>) -> usize {
    if i == 9 {
        return 1;
    }

    let mut s = 0;
    for (x, y, _) in map.four_connected_point(pos) {
        if map[(x, y)] == i + 1 {
            s += solve2((x, y), i + 1, map);
        }
    }
    s
}

fn parse() -> Vec2d<u8> {
    let mut h: usize = 0;
    let k = INPUT
        .lines()
        .inspect(|_| h += 1)
        .map(|x| x.bytes().map(|c| c - b'0'))
        .flatten()
        .collect();

    Vec2d::from_vec_height(k, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (472 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (969 as usize).solution());
    }
}
