use crate::utils::solution::{IntoSolution, Solution};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;

pub const TITLE: &str = "Reactor";
const INPUT: &'static str = include_str!("input.txt");

type Device = u16;

pub fn part1() -> Option<Solution> {
    let (g, conv) = parse();
    let you = *conv.get(&encode(b'y', b'o', b'u')).unwrap();
    let out = *conv.get(&encode(b'o', b'u', b't')).unwrap();

    dfs(you, out, &g, &mut vec![None; g.len()]).solution()
}

pub fn part2() -> Option<Solution> {
    let (g, conv) = parse();
    let svr = *conv.get(&encode(b's', b'v', b'r')).unwrap();
    let out = *conv.get(&encode(b'o', b'u', b't')).unwrap();
    let dac = *conv.get(&encode(b'd', b'a', b'c')).unwrap();
    let fft = *conv.get(&encode(b'f', b'f', b't')).unwrap();

    let mut a = dfs(dac, fft, &g, &mut vec![None; g.len()]);

    if a > 0 {
        a *= dfs(svr, dac, &g, &mut vec![None; g.len()])
            * dfs(fft, out, &g, &mut vec![None; g.len()]);
    } else {
        a = dfs(svr, fft, &g, &mut vec![None; g.len()])
            * dfs(fft, dac, &g, &mut vec![None; g.len()])
            * dfs(dac, out, &g, &mut vec![None; g.len()]);
    }

    a.solution()
}

fn dfs(start: Device, end: Device, g: &Vec<Vec<Device>>, cache: &mut Vec<Option<usize>>) -> usize {
    if start == end {
        return 1;
    }

    if let Some(s) = cache[start as usize] {
        return s;
    }

    let mut s = 0;
    for &n in &g[start as usize] {
        s += dfs(n, end, g, cache);
    }

    cache[start as usize] = Some(s);

    s
}

fn parse() -> (Vec<Vec<Device>>, HashMap<Device, Device>) {
    let mut g: Vec<Vec<Device>> = Vec::new();
    let mut conv = HashMap::default();
    let mut count: u16 = 0;

    for line in INPUT.lines() {
        let mut bytes = line.bytes();

        let a = encode(
            bytes.next().unwrap(),
            bytes.next().unwrap(),
            bytes.next().unwrap(),
        );

        let i = if let Some(i) = conv.get(&a) {
            *i
        } else {
            conv.insert(a, count);
            g.push(Vec::new());
            count += 1;
            count - 1
        };

        while let Some(c) = bytes.next() {
            if c.is_ascii_alphabetic() {
                let b = encode(c, bytes.next().unwrap(), bytes.next().unwrap());

                if let Some(j) = conv.get(&b) {
                    g[i as usize].push(*j);
                } else {
                    conv.insert(b, count);
                    g.push(Vec::new());
                    g[i as usize].push(count);
                    count += 1;
                }
            }
        }
    }

    (g, conv)
}

#[inline]
fn encode(a: u8, b: u8, c: u8) -> Device {
    (a - b'a') as Device * (26 * 26) + (b - b'a') as Device * 26 + (c - b'a') as Device
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (585 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (349322478796032 as usize).solution());
    }
}
