#![allow(dead_code)]

use crate::utils;

pub fn d4_1() {
    let reader = utils::Reader::load_input("src/day4/input.txt").unwrap();
    let overlaps: u32 = reader
        .map(|a| {
            let k: Vec<i32> = a.unwrap()
                .split_terminator(&[',', '-'])
                .map(|a| a.trim().parse().unwrap())
                .collect();
            (
                (k[0] - k[2] >= 0 && k[1] - k[3] <= 0)
                || (k[0] - k[2] <= 0 && k[1] - k[3] >= 0)
            ) as u32
        })
        .sum();
    
    println!("{}", overlaps);
}

pub fn d4_2() {
    let reader = utils::Reader::load_input("src/day4/input.txt").unwrap();
    let overlaps: u32 = reader
        .map(|a| {
            let k: Vec<i32> = a.unwrap()
                .split_terminator(&[',', '-'])
                .map(|a| a.trim().parse().unwrap())
                .collect();
            (
                (k[0] >= k[2] && k[0] <= k[3])
                || (k[1] >= k[2] && k[1] <= k[3])
                || (k[2] >= k[0] && k[2] <= k[1])
                || (k[3] >= k[0] && k[3] <= k[1])
            ) as u32
        })
        .sum();
    
    println!("{}", overlaps);
}