use std::collections::HashSet;

use crate::utils;

pub fn d20() {
    d20_1();
    d20_2();
}

pub fn d20_1() {
    let mut file = Vec::<Number>::new();
    parse("src/day20/input.txt", &mut file);
    let mut visited = HashSet::<usize>::new();
    let len = file.len() as i64;

    let mut i: i64 = 0;
    loop {
        if !visited.insert(file[(i % len) as usize].id) {
            i = (i + 1) % len;
            continue;
        }

        let removed = file.remove((i % len) as usize);
        let new_idx = (((i + removed.value) % (len - 1)) + (len - 1)) % (len - 1);
        file.insert(new_idx as usize, removed);
        
        if visited.len() == file.len() {
            break;
        }
    }

    let grove = [1000, 2000, 3000];
    let pos = file.iter().position(|x| x.value == 0).unwrap() as usize;
    let sum: i64 = grove.iter()
        .map(|x| file[(pos + x) % len as usize].value)
        .inspect(|x| { dbg!(x); })
        .sum();
    println!("{}", sum);
}

pub fn d20_2() {
    let mut file = Vec::<Number>::new();
    parse("src/day20/input.txt", &mut file);
    file.iter_mut().for_each(|x| x.value *= 811589153);

    let mut visited = HashSet::<usize>::new();
    let len = file.len() as i64;
    let pos = (0..len as usize).collect::<Vec<usize>>();
    let mut i: i64 = 0;
    for _ in 0..10 {
        let mut j = 0;
        loop {
            while file[(i % len) as usize].id != pos[j as usize] {
                i = (i + 1) % len;
                continue;
            }
    
            if !visited.insert(file[(i % len) as usize].id) {
                i = (i + 1) % len;
                continue;
            }

            let removed = file.remove((i % len) as usize);
            let new_idx = (((i + removed.value) % (len - 1)) + (len - 1)) % (len - 1);
            file.insert(new_idx as usize, removed);
            j = (j + 1) % len as usize;

            if visited.len() == file.len() {
                visited.clear();
                break;
            }
        }
    }

    let grove = [1000, 2000, 3000];
    let pos = file.iter().position(|x| x.value == 0).unwrap() as usize;
    let sum: i64 = grove.iter()
        .map(|x| file[(pos + x) % len as usize].value)
        .inspect(|x| { dbg!(x); })
        .sum();
    println!("{}", sum);
}

fn parse(file: &str, numbers: &mut Vec<Number>) {
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    buffer.lines()
        .enumerate()
        .for_each(|(i, x)| {
            numbers.push(Number {
                id: i,
                value: x.parse::<i64>().unwrap()
            });
        });
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Number {
    id: usize,
    value: i64,
}