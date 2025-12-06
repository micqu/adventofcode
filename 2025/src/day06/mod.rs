use crate::utils::{
    Parsable, ParsableNonWhitespaceByte,
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Trash Compactor";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let mut numbers: Vec<_> = INPUT.lines().map(|line| line.bytes()).collect();
    let mut insts = numbers.pop().unwrap();
    let mut s = 0;

    while let Some(inst) = insts.next_non_whitespace_byte() {
        let n = numbers
            .iter_mut()
            .filter_map(|mut x| Parsable::<usize>::next_number(&mut x));

        s += match inst {
            b'*' => n.product::<usize>(),
            _ => n.sum::<usize>(),
        };
    }

    s.solution()
}

pub fn part2() -> Option<Solution> {
    let mut numbers: Vec<_> = INPUT.lines().map(|line| line.bytes()).collect();
    let mut insts = numbers.pop().unwrap();
    let mut s = 0;

    while let Some(inst) = insts.next_non_whitespace_byte() {
        let n = std::iter::from_fn(|| next_column_number(&mut numbers));
        s += match inst {
            b'*' => n.product::<usize>(),
            _ => n.sum::<usize>(),
        };
    }

    s.solution()
}

fn next_column_number<T: Iterator<Item = u8>>(lines: &mut Vec<T>) -> Option<usize> {
    let mut number: usize = 0;
    for line in lines.iter_mut() {
        if let Some(next) = line.next()
            && next != b' '
        {
            number = number * 10 + (next - b'0') as usize;
        }
    }

    (number != 0).then(|| number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (6299564383938 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (11950004808442 as usize).solution());
    }
}
