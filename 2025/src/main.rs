#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    collections::{HashMap, HashSet},
    env::{self, Args},
    time::{Duration, Instant},
    vec,
};

use itertools::Itertools;
use utils::solution::Solution;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod utils;

fn main() {
    let args = env::args();

    let mut days = get_day_selection(args);
    let days_sorted = sort_days(&mut days);

    let mut total_runtime = Duration::new(0, 0);
    for (day, parts) in days_sorted {
        let mut contains_solution = false;
        let mut day_runtime = Duration::new(0, 0);

        for part in parts {
            let (part_solver, title) = get_solvers(day.0);
            let part_runtime = Instant::now();

            if let Some(s) = part_solver[part.0]() {
                if !contains_solution {
                    println!("\n=== Day {:02}  {} ===", day.0, title);
                    contains_solution = true;
                }

                let elapsed = part_runtime.elapsed();
                println!("  · Part {}: {}  ({:?})", part.0 + 1, s, elapsed);
                day_runtime += elapsed;
            }
        }
        
        if contains_solution {
            println!("  . Elapsed: {:.4?} ms", day_runtime.as_nanos() as f64 / 1_000_000.0);
        }
        
        total_runtime += day_runtime;
    }

    println!("\nTotal time: {:?}", total_runtime);
}

fn get_day_selection(args: Args) -> HashMap<Day, HashSet<Part>> {
    let mut days = HashMap::<Day, HashSet<Part>>::new();

    for arg in args.skip(1) {
        if !arg.contains('.') {
            let day = Day(arg.parse().unwrap());
            let parts = [Part(0), Part(1)];
            days.entry(day).or_insert(HashSet::from(parts));
        } else {
            let mut t = arg.split('.');
            let day = Day(t.next().unwrap().parse().unwrap());
            let parts = vec![Part(t.next().unwrap().parse::<usize>().unwrap() - 1)];
            let entry = days.entry(day).or_insert(HashSet::new());
            for part in parts {
                entry.insert(part);
            }
        }
    }

    if days.is_empty() {
        for ds in 1..=12 {
            days.insert(Day(ds), HashSet::from([Part(0), Part(1)]));
        }
    }

    days
}

fn sort_days(days: &mut HashMap<Day, HashSet<Part>>) -> Vec<(&Day, Vec<&Part>)> {
    let mut days_sorted = days
        .iter()
        .map(|p| {
            let mut parts = p.1.iter().collect_vec();
            parts.sort_unstable();
            (p.0, parts)
        })
        .collect_vec();

    days_sorted.sort_unstable();
    
    days_sorted
}

fn get_solvers(day: usize) -> (&'static [fn() -> Option<Solution>], &'static str) {
    match day {
        1 => (&[day01::part1, day01::part2], day01::TITLE),
        2 => (&[day02::part1, day02::part2], day02::TITLE),
        3 => (&[day03::part1, day03::part2], day03::TITLE),
        4 => (&[day04::part1, day04::part2], day04::TITLE),
        5 => (&[day05::part1, day05::part2], day05::TITLE),
        6 => (&[day06::part1, day06::part2], day06::TITLE),
        7 => (&[day07::part1, day07::part2], day07::TITLE),
        8 => (&[day08::part1, day08::part2], day08::TITLE),
        9 => (&[day09::part1, day09::part2], day09::TITLE),
        10 => (&[day10::part1, day10::part2], day10::TITLE),
        11 => (&[day11::part1, day11::part2], day11::TITLE),
        12 => (&[day12::part1, day12::part2], day12::TITLE),
        _ => unimplemented!(),
    }
}

#[derive(Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Day(usize);

#[derive(Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Part(usize);
