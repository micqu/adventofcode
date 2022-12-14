use core::panic;

use crate::utils;

pub fn d10() {
    d10_1();
    d10_2();
}

pub fn d10_1() {
    let mut x: i32 = 1;
    let mut cycle = 1;
    let mut sum = 0;

    let mut reader = utils::Reader::load_input("src/day10/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let cmd = line.unwrap().trim().split_whitespace().collect::<Vec<&str>>();
        
        if (cycle - 20) % 40 == 0 {
            sum += cycle * x;
        }

        match cmd[0] {
            "addx" => {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    sum += cycle * x;
                }
                x += cmd[1].parse::<i32>().unwrap();
            },
            "noop" => {},
            _ => panic!()
        }

        cycle += 1;
    }

    println!("{}", sum);
}

pub fn d10_2() {
    let mut x: i32 = 1;
    let mut cycle = 1;
    let mut reader = utils::Reader::load_input("src/day10/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let cmd = line.unwrap().trim().split_whitespace().collect::<Vec<&str>>();
        
        draw(cycle, x);
        
        match cmd[0] {
            "addx" => {
                cycle += 1;
                draw(cycle, x);
                x += cmd[1].parse::<i32>().unwrap();
            },
            "noop" => {},
            _ => panic!()
        }

        cycle += 1;
    }
}

fn draw(cycle: i32, x: i32) {
    let crt_pos = (cycle - 1) % 40;
    if crt_pos >= x - 1 && crt_pos <= x + 1 {
        print!("#");
    } else {
        print!(" ");
    }

    if cycle % 40 == 0 {
        println!()
    }
}