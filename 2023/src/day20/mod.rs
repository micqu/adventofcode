use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::utils::solution::{IntoSolution, Solution};

pub const TITLE: &str = "Pulse Propagation";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (mut map, inp, out) = parse();
    let mut q = VecDeque::<(usize, bool, usize)>::new();
    let mut h: usize = 0;
    let mut l: usize = 0;
    for _ in 0..1000 {
        q.push_back((15625, false, 15625));
        while let Some((i, pulse, from)) = q.pop_front() {
            if pulse {
                h += 1;
            } else {
                l += 1;
            }

            let c = &mut map[i];
            if let Some(t) = &mut c.module_type {
                match t {
                    ModuleType::NOT(state) => {
                        if !pulse {
                            *state = !*state;
                            for o in out[&i].iter() {
                                q.push_back((*o, *state, i));
                            }
                        }
                    }
                    ModuleType::NAND(b) => {
                        b[from] = pulse;
                        let all_high = inp[&i].iter().all(|x| b[*x]);
                        for o in out[&i].iter() {
                            q.push_back((*o, !all_high, i));
                        }
                    }
                }
            } else if let Some(k) = out.get(&i) {
                for o in k.iter() {
                    q.push_back((*o, false, i));
                }
            }
        }
    }
    (h * l).solution()
}

pub fn part2() -> Option<Solution> {
    let (mut map, inp, out) = parse();
    let mut q = VecDeque::<(usize, bool, usize)>::new();
    let rx = (b'r' - b'a') as usize * 25 + (b'x' - b'a') as usize;
    let fin = inp[&rx].iter().next().unwrap();
    let fin_inp = inp[fin].clone();
    let mut ok = vec![0; fin_inp.len()];
    let mut j = 0;
    loop {
        q.push_back((15625, false, 15625));
        j += 1;
        while let Some((i, pulse, from)) = q.pop_front() {
            if ok.iter().all(|x| *x != 0) {
                return ok.iter().product::<usize>().solution();
            }

            let c = &mut map[i];
            if let Some(t) = &mut c.module_type {
                match t {
                    ModuleType::NOT(state) => {
                        if !pulse {
                            *state = !*state;
                            for o in out[&i].iter() {
                                q.push_back((*o, *state, i));
                            }
                        }
                    }
                    ModuleType::NAND(state) => {
                        state[from] = pulse;
                        let all_high = inp[&i].iter().all(|x| state[*x]);

                        if !all_high {
                            for (k, f) in fin_inp.iter().enumerate() {
                                if i == *f && ok[k] == 0 {
                                    ok[k] = j;
                                }
                            }
                        }

                        for o in out[&i].iter() {
                            q.push_back((*o, !all_high, i));
                        }
                    }
                }
            } else if let Some(k) = out.get(&i) {
                for o in k.iter() {
                    q.push_back((*o, false, i));
                }
            }
        }
    }
}

fn parse() -> (
    Vec<Module>,
    HashMap<usize, Vec<usize>>,
    HashMap<usize, Vec<usize>>,
) {
    let mut map: Vec<Module> = (0..15626).map(|id| Module::new(id)).collect_vec();
    let mut inp = HashMap::<usize, Vec<usize>>::new();
    let mut out = HashMap::<usize, Vec<usize>>::new();
    for line in INPUT.lines() {
        let mut bytes = line.bytes();

        let mut t: Option<ModuleType> = None;
        match bytes.next().unwrap() {
            b'%' => t = Some(ModuleType::NOT(false)),
            b'&' => t = Some(ModuleType::NAND(vec![false; 15625])),
            _ => {}
        }

        let mut id: usize = 15625;
        if t.is_some() {
            id = 0;
            while let Some(byte) = bytes.next() {
                if byte == b' ' {
                    break;
                }
                id = id * 25 + (byte - b'a') as usize;
            }
            bytes.nth(2);
        } else {
            bytes.nth(13);
        }

        map[id].module_type = t;

        let mut vs = Vec::new();
        loop {
            let mut id: usize = 0;
            while let Some(byte) = bytes.next() {
                if byte == b',' {
                    break;
                }
                id = id * 25 + (byte - b'a') as usize;
            }
            vs.push(id);

            if let None = bytes.next() {
                break;
            }
        }

        for v in vs.iter() {
            inp.entry(*v).or_insert(Vec::new()).push(id);
        }

        out.entry(id).or_insert(vs);
    }

    (map, inp, out)
}

#[derive(Debug)]
struct Module {
    id: usize,
    module_type: Option<ModuleType>,
}

impl Module {
    fn new(id: usize) -> Self {
        Self {
            id,
            module_type: None,
        }
    }
}

#[derive(Debug)]
enum ModuleType {
    NOT(bool),       // Flip-flop
    NAND(Vec<bool>), // Conjunction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 731517480),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 244178746156661),
            _ => panic!(),
        }
    }
}
