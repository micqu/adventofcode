use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable,
};

pub const TITLE: &str = "Aplenty";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (map, parts) = parse();
    let mut s = 0;
    for part in parts {
        let mut c = ((b'i' as u32) << 8) + b'n' as u32;
        let mut node = map.get(&c).unwrap();
        loop {
            if let Some(n) = map.get(&c) {
                node = n;
            }

            c = node.end;
            for rule in node.rules.iter() {
                if rule.evaluate(&part) {
                    c = rule.next;
                    break;
                }
            }

            if c == b'A' as u32 {
                s += part.x + part.m + part.a + part.s;
                break;
            }

            if c == b'R' as u32 {
                break;
            }
        }
    }
    s.solution()
}

pub fn part2() -> Option<Solution> {
    let map = parse_rules(&mut INPUT.lines());
    let c = ((b'i' as u32) << 8) + b'n' as u32;
    let p = PartRange {
        x: Range::new(1, 4000),
        m: Range::new(1, 4000),
        a: Range::new(1, 4000),
        s: Range::new(1, 4000),
    };
    solve(c, Some(p), &map).unwrap().solution()
}

fn solve(c: u32, mut part: Option<PartRange>, map: &HashMap<u32, Node>) -> Option<usize> {
    let mut s = 0;

    if part.is_none() {
        return Some(s);
    }

    if let Some(p) = part {
        if c == b'A' as u32 {
            s += p.x.len() * p.m.len() * p.a.len() * p.s.len();
            return Some(s);
        }
    }

    if c == b'R' as u32 {
        return None;
    }

    let node = map.get(&c).unwrap();
    for rule in node.rules.iter() {
        if let Some(p) = part {
            let (a, b) = rule.diff(&p);
            if let Some(r) = solve(rule.next, a, map) {
                s += r;
            }
            part = b;
        } else {
            break;
        }
    }

    if let Some(r) = solve(node.end, part, map) {
        s += r;
    }

    Some(s)
}

fn parse_rules(lines: &mut std::str::Lines) -> HashMap<u32, Node> {
    let mut map = HashMap::<u32, Node>::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut secs = line.split(&['{', ',', '}']);

        let mut id_sec = secs.next().unwrap().bytes();
        let mut id: u32 = 0;
        while let Some(byte) = id_sec.next() {
            if byte == b'{' {
                break;
            }
            id = ((id as u32) << 8) + byte as u32;
        }

        let secs = secs.collect_vec();
        let (rem_secs, last) = secs.split_at(secs.len() - 2);
        let mut rules = Vec::<Rule>::new();
        for &sec in rem_secs {
            let mut sec_bytes = sec.bytes();

            let cat = match sec_bytes.next().unwrap() {
                b'x' => Category::X,
                b'm' => Category::M,
                b'a' => Category::A,
                b's' => Category::S,
                _ => panic!(),
            };
            let sec_op = sec_bytes.next().unwrap();
            let n = sec_bytes.next_number().unwrap();
            let op = match sec_op {
                b'<' => Operation::LessThan(cat, n),
                b'>' => Operation::GreaterThan(cat, n),
                _ => panic!(),
            };

            let mut next_id: u32 = 0;
            while let Some(byte) = sec_bytes.next() {
                if byte == b'{' {
                    break;
                }
                next_id = ((next_id as u32) << 8) + byte as u32;
            }

            rules.push(Rule { op, next: next_id })
        }

        let mut end_sec = last.iter().next().unwrap().bytes();
        let mut end: u32 = 0;
        while let Some(byte) = end_sec.next() {
            if byte == b'}' {
                break;
            }
            end = ((end as u32) << 8) + byte as u32;
        }

        map.insert(id, Node { rules, end });
    }

    map
}

fn parse() -> (HashMap<u32, Node>, Vec<Part>) {
    let mut lines = INPUT.lines();
    let map = parse_rules(&mut lines);

    let mut parts = Vec::<Part>::new();
    while let Some(line) = lines.next() {
        let mut bytes = line.bytes();
        parts.push(Part {
            x: bytes.next_number().unwrap(),
            m: bytes.next_number().unwrap(),
            a: bytes.next_number().unwrap(),
            s: bytes.next_number().unwrap(),
        })
    }

    (map, parts)
}

#[derive(Debug)]
struct Node {
    rules: Vec<Rule>,
    end: u32,
}

#[derive(Debug)]
struct Rule {
    op: Operation,
    next: u32,
}

impl Rule {
    fn evaluate(&self, part: &Part) -> bool {
        match &self.op {
            Operation::LessThan(cat, val) => match cat {
                Category::X => part.x < *val,
                Category::M => part.m < *val,
                Category::A => part.a < *val,
                Category::S => part.s < *val,
            },
            Operation::GreaterThan(cat, val) => match cat {
                Category::X => part.x > *val,
                Category::M => part.m > *val,
                Category::A => part.a > *val,
                Category::S => part.s > *val,
            },
        }
    }

    fn diff(&self, part: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        match &self.op {
            Operation::LessThan(cat, v) => Self::split(*cat, *v, part),
            Operation::GreaterThan(cat, v) => {
                let b = Self::split(*cat, *v + 1, part);
                (b.1, b.0)
            }
        }
    }

    fn split(cat: Category, v: usize, part: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        let (mut a, mut b): (Option<PartRange>, Option<PartRange>) = (None, None);
        match cat {
            Category::X => {
                if let Some(less) = part.x.less(v) {
                    let mut np = part.clone();
                    np.x = less;
                    a = Some(np);
                }
                if let Some(greater) = part.x.greater(v) {
                    let mut np = part.clone();
                    np.x = greater;
                    b = Some(np);
                }
            }
            Category::M => {
                if let Some(less) = part.m.less(v) {
                    let mut np = part.clone();
                    np.m = less;
                    a = Some(np);
                }
                if let Some(greater) = part.m.greater(v) {
                    let mut np = part.clone();
                    np.m = greater;
                    b = Some(np);
                }
            }
            Category::A => {
                if let Some(less) = part.a.less(v) {
                    let mut np = part.clone();
                    np.a = less;
                    a = Some(np);
                }
                if let Some(greater) = part.a.greater(v) {
                    let mut np = part.clone();
                    np.a = greater;
                    b = Some(np);
                }
            }
            Category::S => {
                if let Some(less) = part.s.less(v) {
                    let mut np = part.clone();
                    np.s = less;
                    a = Some(np);
                }
                if let Some(greater) = part.s.greater(v) {
                    let mut np = part.clone();
                    np.s = greater;
                    b = Some(np);
                }
            }
        }
        (a, b)
    }
}

#[derive(Debug)]
enum Operation {
    LessThan(Category, usize),
    GreaterThan(Category, usize),
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn new(a: usize, b: usize) -> Self {
        Self { min: a, max: b }
    }

    fn len(&self) -> usize {
        self.max - self.min + 1
    }

    fn less(mut self, v: usize) -> Option<Range> {
        if v > self.min && v < self.max {
            self.max = v - 1;
            return Some(self);
        }

        if v < self.min {
            return None;
        }

        Some(self)
    }

    fn greater(mut self, v: usize) -> Option<Range> {
        if v > self.min && v < self.max {
            self.min = v;
            return Some(self);
        }

        if v > self.max {
            return None;
        }

        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 346230),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 124693661917133),
            _ => panic!(),
        }
    }
}
