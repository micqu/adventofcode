#![allow(dead_code)]

use std::time::SystemTime;

mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod utils;

fn main() {
    let start = SystemTime::now();

    day7::d7();
    day8::d8();
    day9::d9();
    day10::d10();
    day11::d11();
    day12::d12();
    day13::d13();
    day14::d14();
    day15::d15();
    day16::d16();
    day17::d17();
    day18::d18();
    day19::d19();
    day20::d20();
    day21::d21();
    day22::d22();
    day23::d23();
    day24::d24();
    day25::d25();

    println!("\nTotal time: {:?}", start.elapsed().unwrap());
}