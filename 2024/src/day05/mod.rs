use itertools::Itertools;

use crate::utils::{
    solution::{IntoSolution, Solution},
    ToNumbers,
};

pub const TITLE: &str = "Print Queue";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let (rules, updates) = parse();
    let rule_map = create_rulemap(&rules);

    updates
        .iter()
        .filter(|x| solve(&mut Vec::new(), &rule_map, &x, 0))
        .map(|x| x[x.len() / 2 as usize])
        .sum::<usize>()
        .solution()
}

pub fn part2() -> Option<Solution> {
    let (rules, updates) = parse();
    let rule_map = create_rulemap(&rules);
    
    updates
        .iter()
        .filter(|x| !solve(&mut Vec::new(), &rule_map, &x, 0))
        .filter_map(|x| {
            let r = solve2(rule_map.clone(), &x);
            r.0.then(|| r.1)
        })
        .map(|x| x[x.len() / 2 as usize])
        .sum::<usize>()
        .solution()
}

fn solve(
    current: &mut Vec<usize>,
    rules: &Vec<Vec<usize>>,
    update: &Vec<usize>,
    progress: usize,
) -> bool {
    if progress == update.len() {
        return true;
    }

    if let Some(c) = current.last() {
        for rule in rules[*c].iter() {
            current.push(*rule);

            if *rule == update[progress] {
                if solve(current, rules, update, progress + 1) {
                    return true;
                }
            }

            current.pop();
        }
    } else {
        current.push(*update.first().unwrap());
        return solve(current, rules, &update, 1);
    }

    false
}

fn solve2(mut rules: Vec<Vec<usize>>, update: &Vec<usize>) -> (bool, Vec<usize>) {
    let mut sorted: Vec<usize> = Vec::new();
    for _ in 0..update.len() {
        for i in update {
            if sorted.contains(i) {
                continue;
            }

            if update.len() == 1 {
                sorted.push(*i);
                break;
            }

            if update
                .iter()
                .filter(|&x| x != i)
                .all(|&j| !rules[j].contains(i))
            {
                sorted.push(*i);
                rules[*i].clear();
            }
        }
    }

    return (sorted.len() == update.len(), sorted);
}

fn parse() -> (Vec<Rule>, Vec<Vec<usize>>) {
    let mut rules = Vec::<Rule>::new();
    let mut updates = Vec::<Vec<usize>>::new();
    let mut lines = INPUT.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let (a, b) = line.split_once('|').unwrap();
        rules.push(Rule {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
        });
    }

    while let Some(line) = lines.next() {
        updates.push(line.to_numbers());
    }

    (rules, updates)
}

fn create_rulemap(rules: &Vec<Rule>) -> Vec<Vec<usize>> {
    let mut map = vec![Vec::<usize>::new(); 100];
    for rule in rules {
        map[rule.a].push(rule.b);
    }
    map
}

#[derive(Debug)]
struct Rule {
    a: usize,
    b: usize,
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
