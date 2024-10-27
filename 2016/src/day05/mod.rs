use md5::{Md5, Digest};

use crate::{IntoSolution, Solution};

pub const TITLE: &str = "How About a Nice Game of Chess?";

pub fn part1() -> Option<Solution> {
    let id = "uqwqemis";
    let mut password = String::new();
    let mut i = 0;
    for _ in 0..8 {
        loop {
            let guess = format!("{}{}", id, i);
            i += 1;
            if let Some(k) = check(&guess) {
                let ch = k.chars().nth(5).unwrap();
                password.push(ch);
                break;
            }
        }
    }

    password.solution()
}

pub fn part2() -> Option<Solution> {
    let id = "uqwqemis";
    let mut password = String::from_iter(vec![' '; 8]);
    let mut i = 0;
    loop {
        let guess = format!("{}{}", id, i);
        i += 1;
        
        if let Some(k) = check(&guess) {
            let ch_idx = k.chars().nth(5).unwrap().to_digit(10);
            let ch = k.chars().nth(6).unwrap();

            if let Some(idx) = ch_idx {
                if let Some(current) = password.chars().nth(idx as usize) {
                    if current == ' ' {
                        password = replace_char_at(&password, idx as usize, ch);
                        // dbg!(password.clone());
                    }
                }
            }            
        }

        if password.chars().all(|c| c.is_alphanumeric()) {
            break;
        }
    }

    password.solution()
}

fn check(str: &str) -> Option<String> {
    let hex = format!("{:x}", Md5::digest(str));
    if hex.starts_with("00000") {
        return Some(hex);
    }
    None
}

fn replace_char_at(s: &str, idx: usize, c: char) -> String {
    let mut r = String::with_capacity(s.len());
    for (i, d) in s.char_indices() {
        r.push(if i == idx { c } else { d });
    }
    r
}