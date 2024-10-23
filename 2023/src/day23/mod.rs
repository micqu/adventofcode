use crate::utils::{
    solution::{IntoSolution, Solution},
    vec2d::{Vec2d, ADJ_FOUR},
};

pub const TITLE: &str = "A Long Walk";
const INPUT: &'static str = include_str!("input.txt");

const L: usize = 141;
const GOAL: usize = L - 2 + (L - 1) * L;

pub fn part1() -> Option<Solution> {
    let map = parse();
    solve((1, 0), 3, &map).solution()
}

pub fn part2() -> Option<Solution> {
    let map = parse();
    let dmap = condense_map((1, 0), 3, &map);
    let mut seen = vec![false; dmap.len()];
    solve2(1, GOAL, &dmap, &mut seen).unwrap().solution()
}

fn parse() -> Vec2d<u8> {
    let mut map = Vec::<u8>::new();
    let mut h = 1;
    for byte in INPUT.bytes() {
        if byte == b'\n' {
            h += 1;
            continue;
        }

        map.push(byte);
    }
    let w = map.len() / h;
    Vec2d::from_vec(map, w)
}

fn solve(pos: (usize, usize), dir: usize, map: &Vec2d<u8>) -> usize {
    let (pos, _, s) = walk_segment(pos, dir, map);

    if pos.0 == map.width - 2 && pos.1 == map.height - 1 {
        return s;
    }

    let mut best = 0;
    for (vx, vy, vd) in map.four_connected(pos.0, pos.1) {
        if let Some(sd) = is_slope(map[(vx, vy)]) {
            if vd == sd {
                best = best.max(solve((vx, vy), sd, map));
            }
        }
    }
    s + best + 1
}

fn solve2(
    i: usize,
    goal: usize,
    dmap: &Vec<Option<Junction>>,
    seen: &mut Vec<bool>,
) -> Option<usize> {
    if i == goal {
        return Some(0);
    }

    seen[i] = true;

    let mut best: Option<usize> = None;
    if let Some(c) = &dmap[i] {
        for &(nid, s) in c.steps.iter() {
            if s == 0 || seen[nid] {
                continue;
            }

            if let Some(b) = solve2(nid, goal, dmap, seen) {
                best = Some(best.map_or(s + b, |v| v.max(s + b)));
            }
        }
    }

    seen[i] = false;
    best
}

fn condense_map(pos: (usize, usize), dir: usize, map: &Vec2d<u8>) -> Vec<Option<Junction>> {
    let mut dense: Vec<Option<Junction>> = vec![None; map.width * map.height];
    let mut q = Vec::<((usize, usize), usize)>::new();
    q.push((pos, dir));

    while let Some((u, dir)) = q.pop() {
        let (np, nd, s) = walk_segment(u, dir, map);
        let cid = u.0 + u.1 * map.width;
        let nid = np.0 + np.1 * map.width;

        let c = dense[cid].get_or_insert(Junction {
            id: cid,
            steps: vec![(0, 0); 4],
        });
        c.steps[dir] = (nid, s);

        let n = dense[nid].get_or_insert(Junction {
            id: nid,
            steps: vec![(0, 0); 4],
        });
        n.steps[(nd + 2) % 4] = (cid, s);

        for (vx, vy, vd) in map.four_connected_point(np) {
            if let Some(sd) = is_slope(map[(vx, vy)]) {
                if vd == sd {
                    if n.steps[vd].1 == 0 {
                        q.push((np, vd));
                    }
                }
            }
        }
    }

    dense
}

fn walk_segment(
    mut pos: (usize, usize),
    mut dir: usize,
    map: &Vec2d<u8>,
) -> ((usize, usize), usize, usize) {
    let mut s = 0;

    let vx = (pos.0 as isize + ADJ_FOUR[dir].0) as usize;
    let vy = (pos.1 as isize + ADJ_FOUR[dir].1) as usize;
    pos = (vx, vy);
    s += 1;

    loop {
        if pos.0 == map.width - 2 && pos.1 == map.height - 1 {
            return (pos, dir, s);
        }

        let vx = (pos.0 as isize + ADJ_FOUR[dir].0) as usize;
        let vy = (pos.1 as isize + ADJ_FOUR[dir].1) as usize;

        match map[(vx, vy)] {
            b'.' => {
                pos = (vx, vy);
                s += 1;
            }
            b'#' => {
                if let Some(d) = switch_dir(pos, dir, map) {
                    dir = d;
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    for (vx, vy, vd) in map.four_connected_point(pos) {
        if let Some(_) = is_slope(map[(vx, vy)]) {
            let vx = (vx as isize + ADJ_FOUR[vd].0) as usize;
            let vy = (vy as isize + ADJ_FOUR[vd].1) as usize;
            pos = (vx, vy);
            dir = vd;
            s += 2;
        }
    }

    (pos, dir, s)
}

fn switch_dir(pos: (usize, usize), dir: usize, map: &Vec2d<u8>) -> Option<usize> {
    for (vx, vy, vd) in map.four_connected(pos.0, pos.1) {
        if vd != (dir + 2) % 4 {
            if map[(vx, vy)] == b'.' {
                return Some(vd);
            }
        }
    }
    None
}

fn is_slope(byte: u8) -> Option<usize> {
    match byte {
        b'>' => Some(0),
        b'v' => Some(3),
        _ => None,
    }
}

#[derive(Debug, Clone)]
struct Junction {
    id: usize,
    steps: Vec<(usize, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 2250),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 6470),
            _ => panic!(),
        }
    }
}
