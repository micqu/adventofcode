#![allow(dead_code)]

use std::{collections::VecDeque, cmp::Ordering};

use crate::utils;

pub fn d13_1() {
    let result: u32 = parse("src/day13/inputp1.txt")
        .chunks_mut(2)
        .enumerate()
        .map(|(i, x)| {
            let a = x[0].pop_front().unwrap();
            let b = x[1].pop_front().unwrap();
            let result = solve(&a, &b);
            if result == 2 {
                return 0;
            }
            (i + 1) as u32
        })
        .sum();
    println!("{}", result);
}

pub fn d13_2() {
    let s = parse("src/day13/inputp2.txt");
    let mut es = s
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &VecDeque<El>)>>();
    
        es.sort_by(|a, b| {
        let af = a.1.front().unwrap();
        let bf = b.1.front().unwrap();
        let result = solve(&af, &bf);
        return match result {
            2 => Ordering::Greater,
            1 => Ordering::Less,
            0 => Ordering::Equal,
            _ => panic!()
        }
    });
    let p1 = es.iter().position(|x| x.0 == 0).unwrap() + 1;
    let p2 = es.iter().position(|x| x.0 == 1).unwrap() + 1;
    println!("{}", p1 * p2);
}

fn parse(file: &str) -> Vec::<VecDeque<El>> {
    let mut s = Vec::<VecDeque<El>>::new();
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let line_unwrapped = line.unwrap();

        if line_unwrapped.trim().is_empty() {
            continue;
        }

        let mut st = VecDeque::<El>::new();
        split_keep(line_unwrapped.trim())
            .iter()
            .filter(|&&x| x != ",")
            .map(|x| *x)
            .for_each(|x| {
                match x {
                    "[" => {
                        let new_list = El::List(VecDeque::<El>::new());
                        st.push_back(new_list);
                    },
                    "]" => {
                        if st.len() > 1 {
                            let last = st.pop_back().unwrap();
                            match st.iter_mut().last() {
                                Some(El::List(list)) => list.push_back(last),
                                _ => st.push_back(last)
                            }
                        }
                    },
                    a => {
                        let n = a.parse::<u32>().unwrap();
                        match st.iter_mut().last() {
                            Some(El::List(list)) => list.push_back(El::Number(n)),
                            _ => st.push_back(El::Number(n))
                        }
                    }
                }
            });
        s.push(st);
    }
    s
}

fn split_keep<'a>(text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(|c: char| !(c.is_alphanumeric() || c == '\'')) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

fn solve(a: &El, b: &El) -> u32 {
    match a.clone() {
        El::List(mut alist) => {
            match b.clone() {
                El::List(mut blist) => {
                    let mut result = 0;
                    while result == 0 {
                        let af = alist.pop_front();
                        let bf = blist.pop_front();
                        let at = !af.is_none() as i32;
                        let bt = !bf.is_none() as i32;
                        if at == 0 && bt == 0 {
                            return 0;
                        }
                        if at == 0 && bt == 1 {
                            return 1;
                        }
                        if at == 1 && bt == 0 {
                            return 2;
                        }
                        result = solve(&af.unwrap(), &bf.unwrap());
                    }
                    return result;
                },
                El::Number(bn) => {
                    let mut new_list = VecDeque::new();
                    new_list.push_back(El::Number(bn));
                    return solve(a, &El::List(new_list));
                }
            }
        },
        El::Number(an) => {
            match b.clone() {
                El::List(_) => {
                    let mut new_list = VecDeque::new();
                    new_list.push_back(El::Number(an));
                    return solve(&El::List(new_list), b);
                },
                El::Number(bn) => {
                    if an == bn {
                        return 0;
                    }
                    if an < bn {
                        return 1;
                    }
                    return 2;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum El {
    List(VecDeque<El>),
    Number(u32)
}