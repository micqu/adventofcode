use crate::utils::{
    solution::{IntoSolution, Solution},
    Parsable, ToNumbers,
};

pub const TITLE: &str = "Print Queue";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (rule_map, updates) = parse();
    updates
        .iter()
        .filter(|x| solve(&rule_map, &x))
        .map(|x| x[x.len() / 2 as usize])
        .sum::<usize>()
        .solution()
}

pub fn part2() -> Option<Solution> {
    let (rule_map, mut updates) = parse();
    updates
        .iter_mut()
        .filter(|x| !solve(&rule_map, &x))
        .map(|x| solve2(&rule_map, x))
        .sum::<usize>()
        .solution()
}

fn solve(rules: &Vec<Vec<usize>>, update: &Vec<usize>) -> bool {
    for i in 0..update.len() - 1 {
        if !rules[update[i]].contains(&update[i + 1]) {
            return false;
        }
    }
    true
}

fn solve2(rules: &Vec<Vec<usize>>, update: &mut Vec<usize>) -> usize {
    let mut last = 0;
    for _ in 0..=update.len() / 2 {
        let pos = update
            .iter()
            .position(|&p| {
                let rule = &rules[p];
                update
                    .iter()
                    .filter(|j| **j != p)
                    .all(|j| !rule.contains(j))
            })
            .unwrap();
        last = update.remove(pos);
    }
    last
}

fn parse() -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut map = vec![Vec::<usize>::new(); 100];
    let mut updates = Vec::<Vec<usize>>::new();
    let mut lines = INPUT.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut l = line.bytes();
        let a: usize = l.next_number().unwrap();
        let b: usize = l.next_number().unwrap();
        map[a].push(b);
    }

    while let Some(line) = lines.next() {
        updates.push(line.to_numbers());
    }

    (map, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = super::part1().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 4814),
            _ => panic!(),
        }
    }

    #[test]
    fn part2() {
        let result = super::part2().unwrap();
        match result {
            Solution::Usize(a) => assert_eq!(a, 5448),
            _ => panic!(),
        }
    }
}
