use crate::utils::solution::{Solution, IntoSolution};

const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (seeds, maps) = parse();
    seeds.iter().map(|&seed| {
        let mut v = seed;
        for map in maps.iter() {            
            for mapping in map.iter() {
                if mapping.contains(v) {
                    v += mapping.delta();
                    break;
                }
            }
        }
        v
    }).min().unwrap().solution()
}

pub fn part2() -> Option<Solution> {
    let (seeds, maps) = parse();
    let mut s = i64::MAX;
    for (a, b) in seeds.chunks(2).map(|x| (x[0], x[0] + x[1] - 1)) {
        let locations = maps.iter().fold(vec![(a, b)], |rs, map| {
            rs.iter().flat_map(|(a, b)| pass(*a, *b, &map)).collect()
        });
        
        s = s.min(locations.iter().map(|(a, _)| *a).min().unwrap());
    }

    s.solution()
}

fn pass(a: i64, b: i64, map: &Vec<Mapping>) -> Vec<(i64, i64)> {
    let mut s = Vec::<(i64, i64)>::new();
    let mut parts = vec![(a, b)];
    let mut rem = Vec::<(i64, i64)>::new();

    for mapping in map {
        let d = mapping.delta();
        while let Some((a, b)) = parts.pop() {
            if let Some((x0, y0, y1, x1)) = mapping.overlap(a, b) {
                s.push((y0 + d, y1 + d));

                if !mapping.contains(a) {
                    rem.push((x0, y0 - 1));
                }

                if !mapping.contains(b) {
                    rem.push((y1 + 1, x1));
                }
            } else {
                rem.push((a, b));
            }
        }

        parts.append(&mut rem);
    }
    
    s.append(&mut parts);
    s
}

fn parse() -> (Vec::<i64>, Vec::<Vec<Mapping>>) {
    let mut seeds = Vec::<i64>::new();
    let mut maps = Vec::<Vec<Mapping>>::new();

    let mut t = 0;
    for line in INPUT.lines() {
        if line.ends_with(':') {
            maps.push(Vec::new());
            t += 1;
            continue;
        }

        if line.is_empty() {
            continue;
        }

        match t {
            0 => seeds.extend(line.split(' ').skip(1).map(|x| x.parse::<i64>().unwrap())),
            _ => {
                let mut mapping = line.split(' ');
                let dst: i64 = mapping.next().unwrap().parse().unwrap();
                let src: i64 = mapping.next().unwrap().parse().unwrap();
                let len: i64 = mapping.next().unwrap().parse().unwrap();
                maps[t - 1].push(Mapping::new(src, dst, len));
            }
        }
    }

    (seeds, maps)
}

#[derive(Debug)]
struct Mapping {
    dst: i64,
    src: i64,
    len: i64,
    end: i64
}

impl Mapping {
    fn new(src: i64, dst: i64, len: i64) -> Self {
        Self { src, dst, len, end: src + len - 1 }
    }

    fn delta(&self) -> i64 {
        self.dst - self.src
    }

    fn contains(&self, n: i64) -> bool {
        n >= self.src && n <= self.end
    }

    fn overlap(&self, a: i64, b: i64) -> Option<(i64, i64, i64, i64)> {
        let min = self.src.max(a);
        let max = self.end.min(b);
        if min <= max {
            return Some((self.src.min(a), min, max, self.end.max(b)));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::I64(a) => assert_eq!(a, 240320250),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::I64(a) => assert_eq!(a, 28580589),
            _ => panic!(),
        }
    }
}