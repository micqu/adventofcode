use crate::utils::{
    solution::{IntoSolution, Solution},
    vec2d::Vec2d,
    Parsable,
};

pub const TITLE: &str = "Sand Slabs";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (mut height, mut cuboids) = parse();
    let (up, down) = fall(&mut height, &mut cuboids);

    let mut s: usize = 0;
    for c in cuboids.iter() {
        if up[c.id].iter().all(|u| down[*u].len() > 1) {
            s += 1;
        }
    }
    s.solution()
}

pub fn part2() -> Option<Solution> {
    let (mut height, mut cuboids) = parse();
    let (up, down) = fall(&mut height, &mut cuboids);

    let mut s = 0;
    for c in cuboids.iter() {
        for &u in up[c.id].iter() {
            let mut fallen = vec![0; cuboids.len() + 1];
            if down[u].len() == 1 {
                disintegrate(c.id, &up, &mut down.clone(), &mut fallen);
                s += fallen.iter().sum::<usize>();
                break;
            }
        }
    }
    s.solution()
}

fn disintegrate(i: usize, up: &Vec<Vec<usize>>, down: &mut Vec<Vec<usize>>, fallen: &mut Vec<usize>) {
    for &u in up[i].iter() {
        if down[u].len() == 1 {
            fallen[u] = 1;
            disintegrate(u, up, down, fallen);
        } else {
            down[u].remove(0);
        }
    }
}

fn fall(height: &mut Vec2d<usize>, cuboids: &mut Vec<Cuboid>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut ids = Vec2d::<usize>::from_vec(vec![0; height.data.len()], height.width);
    let mut up = vec![Vec::<usize>::new(); cuboids.len() + 1];
    let mut down = vec![Vec::<usize>::new(); cuboids.len() + 1];
    cuboids.sort_unstable();

    for c in cuboids.iter_mut() {
        let mut dmin = usize::MAX;
        for x in c.a.x..=c.b.x {
            for y in c.a.y..=c.b.y {
                for z in c.a.z..=c.b.z {
                    dmin = dmin.min(z - height[(x, y)]);
                }
            }
        }
        c.a.z -= dmin - 1;
        c.b.z -= dmin - 1;
        let mut used = Vec::<usize>::new();
        for x in c.a.x..=c.b.x {
            for y in c.a.y..=c.b.y {
                for z in c.a.z..=c.b.z {
                    if z - height[(x, y)] == 1 {
                        let pid = ids[(x, y)];
                        if c.id != pid && !used.contains(&pid) {
                            up[pid].push(c.id);
                            down[c.id].push(pid);
                            used.push(pid);
                        }
                    }
                    height[(x, y)] = z;
                    ids[(x, y)] = c.id;
                }
            }
        }
    }
    (up, down)
}

fn parse() -> (Vec2d<usize>, Vec<Cuboid>) {
    let mut cuboids = Vec::new();
    let mut xmax: usize = 0;
    let mut ymax: usize = 0;
    for (i, line) in INPUT.lines().enumerate() {
        let mut bytes = line.bytes();
        let a = read_point(&mut bytes);
        let b = read_point(&mut bytes);
        xmax = xmax.max(b.x);
        ymax = ymax.max(b.y);
        cuboids.push(Cuboid { id: i + 1, a, b });
    }

    let w = xmax + 1;
    let h = ymax + 1;
    let height = Vec2d::from_vec(vec![0; w * h], w);
    (height, cuboids)
}

fn read_point<T>(a: &mut T) -> Point
where
    T: Iterator<Item = u8>,
{
    Point {
        x: a.next_number().unwrap(),
        y: a.next_number().unwrap(),
        z: a.next_number().unwrap(),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Cuboid {
    id: usize,
    a: Point,
    b: Point,
}

impl PartialOrd for Cuboid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Cuboid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.a.z.cmp(&other.a.z)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 463),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 89727),
            _ => panic!(),
        }
    }
}
