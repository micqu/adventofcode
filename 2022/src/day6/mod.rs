#![allow(dead_code)]

use std::{self, collections::HashSet};
use crate::utils;

pub fn d6_1() {
    let len = 4;
    let mut set: HashSet<u8> = HashSet::with_capacity(len);
    let reader = utils::Reader::load_input("src/day6/input.txt").unwrap();
    let k: usize = reader
        .map(|line| {
            line.unwrap()
                .trim()
                .as_bytes()
                .windows(len)
                .position(|a| {
                    set.clear();
                    set.extend(a);
                    set.len() == len
                }).unwrap()
    }).next().unwrap() + len;
    
    println!("{}", k);
}

pub fn d6_2() {
    let len = 14;
    let mut set: HashSet<u8> = HashSet::with_capacity(len);
    let reader = utils::Reader::load_input("src/day6/input.txt").unwrap();
    let k: usize = reader
        .map(|line| {
            line.unwrap()
                .trim()
                .as_bytes()
                .windows(len)
                .position(|a| { 
                    set.clear();
                    set.extend(a);
                    set.len() == len
                }).unwrap()
        }).next().unwrap() + len;
    
    println!("{}", k);
}