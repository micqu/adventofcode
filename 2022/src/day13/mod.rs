use std::cmp::Ordering;

use crate::utils;

pub fn d13_1() {
    let result: u32 = parse("src/day13/inputp1.txt")
        .chunks_mut(2)
        .enumerate()
        .map(|(i, x)| {
            if &x[0] < &x[1] {
                return (i + 1) as u32;
            }
            0
        })
        .sum();
    println!("{}", result);
}

pub fn d13_2() {
    let mut s = parse("src/day13/inputp2.txt");
    s.sort_unstable();
    let a = El::List(vec![El::List(vec![El::List(vec![El::Number(2)])])]);
    let b = El::List(vec![El::List(vec![El::List(vec![El::Number(6)])])]);
    let p1 = s.iter().position(|x| *x == a).unwrap() + 1;
    let p2 = s.iter().position(|x| *x == b).unwrap() + 1;
    println!("{}", p1 * p2);
}

fn parse(file: &str) -> Vec::<El> {
    let mut s = Vec::<El>::new();
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let line_unwrapped = line.unwrap();

        if line_unwrapped.trim().is_empty() {
            continue;
        }

        let mut st = Vec::<El>::new();
        split_keep(line_unwrapped.trim())
            .iter()
            .filter(|&&x| x != ",")
            .for_each(|x| match *x {
                "[" => {
                    let new_list = El::List(Vec::<El>::new());
                    st.push(new_list);
                },
                "]" => {
                    if st.len() > 1 {
                        let last = st.pop().unwrap();
                        match st.iter_mut().last() {
                            Some(El::List(list)) => list.push(last),
                            _ => st.push(last)
                        }
                    }
                },
                a => {
                    let n = a.parse::<u32>().unwrap();
                    match st.iter_mut().last() {
                        Some(El::List(list)) => list.push(El::Number(n)),
                        _ => st.push(El::Number(n))
                    }
                }
            });
        s.push(El::List(st));
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum El {
    List(Vec<El>),
    Number(u32)
}

impl PartialOrd for El {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for El {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            El::List(alist) => match other {
                El::List(blist) => {
                    let mut ai = alist.iter();
                    let mut bi = blist.iter();
                    loop {
                        let af = ai.next();
                        let bf = bi.next();
                        let cmp =  bf.is_none().cmp(&af.is_none());
                        if cmp == Ordering::Equal && af.is_none() && bf.is_none() {
                            return cmp;
                        }
                        let result = af.cmp(&bf);
                        if result.is_ne() {
                            return result;
                        }
                    }
                },
                El::Number(bn) => self.cmp(&El::List(vec![El::Number(*bn)]))
            },
            El::Number(an) => match other {
                El::List(_) => El::List(vec![El::Number(*an)]).cmp(&other),
                El::Number(bn) => an.cmp(&bn)
            }
        }
    }
}