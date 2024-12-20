use crate::utils::{solution::{IntoSolution, Solution}, ToDigit};

pub const TITLE: &str = "Mull It Over";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut bytes = INPUT.bytes();
    let p: Vec<_> = "mul(".bytes().collect();
    let mut pos: usize = 0;
    let mut s = 0;
    while let Some(b) = bytes.next() {
        if b == p[pos] {
            pos += 1;
            if pos == 4 {
                if let (Some(a), Some(b',')) = parse(&mut bytes) {
                    if let (Some(b), Some(b')')) = parse(&mut bytes) {
                        s += a * b;
                    }
                }
                pos = 0;
            }
        } else {
            pos = 0;
        }
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let mut bytes = INPUT.bytes();
    let p: Vec<_> = "mul(".bytes().collect();
    let p_enable: Vec<_> = "do()".bytes().collect();
    let p_disable: Vec<_> = "don't()".bytes().collect();
    let mut pos: usize = 0;
    let mut pos_enable: usize = 0;
    let mut pos_disable: usize = 0;
    let mut enable = true;
    let mut s = 0;
    while let Some(b) = bytes.next() {
        if enable && b == p[pos] {
            pos += 1;
            if pos == p.len() {
                if let (Some(a), Some(b',')) = parse(&mut bytes) {
                    if let (Some(b), Some(b')')) = parse(&mut bytes) {
                        s += a * b;
                    }
                }
                pos = 0;
            }
        } else {
            pos = 0;
        }

        if b == p_enable[pos_enable] {
            pos_enable += 1;
            if pos_enable == p_enable.len() {
                enable = true;
                pos_enable = 0;
            }
        } else {
            pos_enable = 0;
        }

        if b == p_disable[pos_disable] {
            pos_disable += 1;
            if pos_disable == p_disable.len() {
                enable = false;
                pos_disable = 0;
            }
        } else {
            pos_disable = 0;
        }
    }

    s.solution()
}

pub fn parse<T>(input: &mut T) -> (Option<usize>, Option<u8>)
where
    T: Iterator<Item = u8>,
{
    let mut value: Option<usize> = None;
    for char in input {
        if let Some(digit) = char.to_digit() {
            if let Some(current) = value {
                value = Some(current * 10 + digit as usize);
            } else {
                value = Some(digit as usize);
            }
        } else {
            return (value, Some(char));
        }
    }

    (value, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1() {
        assert_eq!(super::part1(), (175015740 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (112272912 as usize).solution());
    }
}