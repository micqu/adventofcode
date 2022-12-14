#![allow(dead_code)]

use std::time::SystemTime;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod utils;

fn main() {
    let start = SystemTime::now();
    
    // day1::d1();
    // day2::d2();
    // day3::d3();
    // day4::d4();
    // day5::d5();
    // day6::d6();
    // day7::d7();
    // day8::d8();
    // day9::d9();
    // day10::d10();
    // day11::d11();
    // day12::d12();
    // day13::d13();
    // day14::d14();

    let end = SystemTime::now();
    println!("\nTotal time: {}µs", end.duration_since(start).unwrap().as_micros());
}