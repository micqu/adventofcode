use itertools::Itertools;

use crate::utils::solution::{IntoSolution, Solution};

pub const TITLE: &str = "Crossed Wires";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (mut wires, gates) = parse();
    simulate(&mut wires, &gates);
    get_number(b'z', 46, &wires).solution()
}

pub fn part2() -> Option<Solution> {
    let (wires, gates) = parse();

    // let x = get_number(b'x', 45, &wires);
    // let y = get_number(b'y', 45, &wires);

    let and_gates = gates.iter().filter(|x| x.gate_type == GateType::And).collect_vec();
    let or_gates = gates.iter().filter(|x| x.gate_type == GateType::Or).collect_vec();
    let xor_gates = gates.iter().filter(|x| x.gate_type == GateType::Xor).collect_vec();

    for g in &gates {
        match g.gate_type {
            GateType::And => {
                let (x, _, _) = decode(g.c);
                if x == b'z' {
                    dbg!(g.c);
                }
                let k1 = or_gates.iter().filter(|d| d.a == g.c || d.b == g.c).count() == 1;
                if !k1 {
                    dbg!(g.c);
                }
            },
            GateType::Or => {
                let (x, y, z) = decode(g.c);
                if x == b'z' && y != b'4' && z != b'5' {
                    dbg!(g.c);
                }

                let k1 = and_gates.iter().filter(|d| d.c == g.a);
                if k1.count() != 1 {
                    dbg!(g.a);
                }

                let k2 = and_gates.iter().filter(|d| d.c == g.b);
                if k2.count() != 1 {
                    dbg!(g.b);
                }

                let k3 = xor_gates.iter().filter(|d| d.a == g.c || d.b == g.c).count() == 1;
                let k4 = and_gates.iter().filter(|d| d.a == g.c || d.b == g.c).count() == 1;
                if !k3 || !k4 {
                    dbg!(g.c);
                }
            },
            GateType::Xor => {
                let (x, y, z) = decode(g.c);
                if x == b'z' {
                    continue;
                } else {
                    let k1 = xor_gates.iter().filter(|d| d.a == g.c || d.b == g.c).count() == 1;
                    let k2 = and_gates.iter().filter(|d| d.a == g.c || d.b == g.c).count() == 1;
                    if !k1 || !k2 {
                        dbg!(g.c);
                    }
                }
            },
        }
    }

    None
}

fn simulate(wires: &mut Vec<Option<bool>>, gates: &Vec<Gate>) {
    let mut seen = vec![false; gates.len()];
    let mut done = false;
    while !done {
        done = true;

        for (g, gate) in gates.iter().enumerate() {
            if seen[g] {
                continue;
            }

            done = false;
            match gate.gate_type {
                GateType::And => {
                    if let Some((a, b)) = wires[gate.a].zip(wires[gate.b]) {
                        seen[g] = true;
                        wires[gate.c] = Some(a & b);
                    }
                }
                GateType::Or => {
                    if let Some((a, b)) = wires[gate.a].zip(wires[gate.b]) {
                        seen[g] = true;
                        wires[gate.c] = Some(a | b);
                    }
                }
                GateType::Xor => {
                    if let Some((a, b)) = wires[gate.a].zip(wires[gate.b]) {
                        seen[g] = true;
                        wires[gate.c] = Some(a ^ b);
                    }
                }
            }
        }
    }
}

fn get_number(ch: u8, len: usize, wires: &Vec<Option<bool>>) -> usize {
    let mut output: usize = 0;
    for i in (0..len).rev() {
        let b = ((i / 10) % 10) as u8;
        let c = (i % 10) as u8;
        if wires[encode(ch, b + 48, c + 48)].unwrap() {
            output = output << 1 | 1;
        } else {
            output = output << 1;
        }
    }
    output
}

fn parse() -> (Vec<Option<bool>>, Vec<Gate>) {
    let mut wires = vec![None; 36 * 36 * 36];
    let mut gates = Vec::new();
    let mut lines = INPUT.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut line = line.bytes();
        let wire = get_component(&mut line);
        let v = match line.skip(2).next().unwrap() {
            b'0' => false,
            b'1' => true,
            _ => panic!(),
        };

        wires[wire] = Some(v);
    }

    while let Some(line) = lines.next() {
        let mut split = line.split(' ');

        let i1 = get_component(&mut split.next().unwrap().bytes());
        let gate = split.next().unwrap();
        let i2 = get_component(&mut split.next().unwrap().bytes());
        let output = get_component(&mut split.skip(1).next().unwrap().bytes());

        gates.push(match gate {
            "AND" => Gate { gate_type: GateType::And, a: i1, b: i2, c: output },
            "OR" => Gate { gate_type: GateType::Or, a: i1, b: i2, c: output },
            "XOR" => Gate { gate_type: GateType::Xor, a: i1, b: i2, c: output },
            _ => panic!(),
        });
    }

    (wires, gates)
}

fn get_component<T: Iterator<Item = u8>>(iter: &mut T) -> usize {
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();
    return encode_byte(a) * 36 * 36 + encode_byte(b) * 36 + encode_byte(c);
}

fn encode(a: u8, b: u8, c: u8) -> usize {
    encode_byte(a) * 36 * 36 + encode_byte(b) * 36 + encode_byte(c)
}

fn decode(a: usize) -> (u8, u8, u8) {
    (
        decode_byte(a / 36 / 36),
        decode_byte((a / 36) % 36),
        decode_byte(a % 36),
    )
}

fn encode_byte(a: u8) -> usize {
    if a >= 97 {
        (a - b'a') as usize
    } else {
        (a - b'0') as usize + 26
    }
}

fn decode_byte(a: usize) -> u8 {
    if a >= 26 {
        a as u8 + b'0'
    } else {
        a as u8 + b'a'
    }
}

#[derive(Debug, Clone, PartialEq)]
enum GateType {
    And,
    Or,
    Xor,
}

struct Gate {
    gate_type: GateType,
    a: usize,
    b: usize,
    c: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (36036961805936 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (216772608 as usize).solution());
    }
}
