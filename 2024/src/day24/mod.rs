use crate::utils::solution::{IntoSolution, Solution};

pub const TITLE: &str = "?!";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (mut wires, gates) = parse();
    let mut seen = vec![false; gates.len()];
    let mut done = false;
    while !done {
        done = true;

        for (g, gate) in gates.iter().enumerate() {
            if seen[g] {
                continue;
            }

            done = false;
            match gate {
                Gate::And(a, b, c) => {
                    if let Some((a, b)) = wires[*a].zip(wires[*b]) {
                        seen[g] = true;
                        wires[*c] = Some(a & b);
                    }
                }
                Gate::Or(a, b, c) => {
                    if let Some((a, b)) = wires[*a].zip(wires[*b]) {
                        seen[g] = true;
                        wires[*c] = Some(a | b);
                    }
                }
                Gate::Xor(a, b, c) => {
                    if let Some((a, b)) = wires[*a].zip(wires[*b]) {
                        seen[g] = true;
                        wires[*c] = Some(a ^ b);
                    }
                }
            }
        }
    }

    let nz = 46;
    let mut output: usize = 0;
    for i in (0..nz).rev() {
        let b = (i / 10) % 10;
        let c = i % 10;
        if wires[encode(b'z', b + 48, c + 48)].unwrap() {
            output = output << 1 | 1;
        } else {
            output = output << 1;
        }
    }

    output.solution()
}

pub fn part2() -> Option<Solution> {
    None
}

fn parse() -> (Vec<Option<bool>>, Vec<Gate>) {
    let mut wires = vec![None; 35 * 35 * 35];
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
            "AND" => Gate::And(i1, i2, output),
            "OR" => Gate::Or(i1, i2, output),
            "XOR" => Gate::Xor(i1, i2, output),
            _ => panic!(),
        });
    }

    (wires, gates)
}

fn get_component<T: Iterator<Item = u8>>(iter: &mut T) -> usize {
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();
    return encode_byte(a) * 35 * 35 + encode_byte(b) * 35 + encode_byte(c)
}

fn encode(a: u8, b: u8, c: u8) -> usize {
    encode_byte(a) * 35 * 35 + encode_byte(b) * 35 + encode_byte(c)
}

// fn encode(a: u8, b: u8, c: u8) -> usize {
//     encode_byte(a) * 35 * 35 + encode_byte(b) * 35 + encode_byte(c)
// }

fn encode_byte(a: u8) -> usize {
    if a >= 97 {
        (a - b'a') as usize
    } else {
        (a - b'0') as usize + 25
    }
}

#[derive(Debug)]
enum Gate {
    And(usize, usize, usize),
    Or(usize, usize, usize),
    Xor(usize, usize, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (36035961805936 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (216772608 as usize).solution());
    }
}
