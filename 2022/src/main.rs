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
mod utils;

fn main() {
    let start = SystemTime::now();
    
    // day1::d1_1();
    // day1::d1_2();
    // day2::d2_1();
    // day2::d2_2();
    // day3::d3_1();
    // day3::d3_2();
    // day4::d4_1();
    // day4::d4_2();
    // day5::d5_1();
    // day5::d5_2();
    // day6::d6_1();
    // day6::d6_2();
    // day7::d7_1();
    // day7::d7_2();
    // day8::d8_1();
    // day8::d8_2();
    // day9::d9_1();
    // day9::d9_2();
    // day10::d10_1();
    // day10::d10_2();
    day11::d11_1();
    day11::d11_2();

    let end = SystemTime::now();
    println!("\nTotal time: {}ms", end.duration_since(start).unwrap().as_millis());
}