use crate::utils::{
    solution::{IntoSolution, Solution},
    vec2d::{Vec2d, ADJ_FOUR},
};

pub const TITLE: &str = "The Floor Will Be Lava";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let map = parse();
    let mut seen = Vec2d::from_vec(vec![0; map.data.len()], map.width);
    solve(Position { x: -1, y: 0 }, 0, &map, &mut seen).solution()
}

pub fn part2() -> Option<Solution> {
    let map = parse();
    let mut s = 0;
    for x in 0..map.width as isize {
        let mut seen = Vec2d::from_vec(vec![0; map.data.len()], map.width);
        s = s.max(solve(Position { x, y: -1 }, 3, &map, &mut seen));
        let mut seen = Vec2d::from_vec(vec![0; map.data.len()], map.width);
        s = s.max(solve(Position { x, y: map.height as isize }, 1, &map, &mut seen));
    }
    for y in 0..map.height as isize {
        let mut seen = Vec2d::from_vec(vec![0; map.data.len()], map.width);
        s = s.max(solve(Position { x: -1, y }, 0, &map, &mut seen));
        let mut seen = Vec2d::from_vec(vec![0; map.data.len()], map.width);
        s = s.max(solve(Position { x: map.width as isize, y }, 2, &map, &mut seen));
    }

    s.solution()
}

fn parse() -> Vec2d<u8> {
    let mut map = Vec::<u8>::new();
    for byte in INPUT.bytes() {
        if byte != b'\n' {
            map.push(byte);
        }
    }
    Vec2d::from_vec(map, 110)
}

fn solve(
    mut c: Position,
    mut dir: usize,
    map: &Vec2d<u8>,
    seen: &mut Vec2d<u8>,
) -> u32 {
    let mut s = 0;
    loop {
        let (dx, dy) = ADJ_FOUR[dir];
        let nx = c.x + dx;
        let ny = c.y + dy;

        if nx < 0 || nx >= map.width as isize || ny < 0 || ny >= map.height as isize {
            return s;
        }

        c.x = nx;
        c.y = ny;

        let sd = &mut seen[(c.x as usize, c.y as usize)];
        if *sd & (1 << dir) != 0 {
            return s;
        }
        
        if *sd == 0 {
            s += 1;
        }

        *sd |= 1 << dir;

        match map[(c.x as usize, c.y as usize)] {
            b'/' => {
                if dir % 2 == 0 {
                    dir += 1;
                } else {
                    dir += 3;
                }
            }
            b'\\' => {
                if dir % 2 == 1 {
                    dir += 1;
                } else {
                    dir += 3;
                }
            }
            b'|' => {
                if dir == 0 || dir == 2 {
                    s += solve(c.clone(), 1, map, seen);
                    s += solve(c.clone(), 3, map, seen);
                    return s;
                }
            }
            b'-' => {
                if dir == 1 || dir == 3 {
                    s += solve(c.clone(), 0, map, seen);
                    s += solve(c.clone(), 2, map, seen);
                    return s;
                }
            }
            _ => {}
        }
        dir %= 4;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 7608),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::U32(a) => assert_eq!(a, 8221),
            _ => panic!(),
        }
    }
}
