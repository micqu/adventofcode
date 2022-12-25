use core::panic;

use crate::utils;

pub fn d25() {
    let mut ns = Vec::<Vec<i64>>::new();
    parse("src/day25/input.txt", &mut ns);

    let k = ns.iter().map(|x| {
        x.iter().rev()
            .enumerate()
            .map(|(i, &c)| c * (5i64.pow(i as u32)))
            .sum::<i64>()
    }).collect::<Vec<i64>>();    
    
    let sum = k.iter().sum::<i64>();
    println!("{}", sum);

    let s = to_snafu(sum);
    let d = s.iter().map(|x| {
        match x {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        }
    }).collect::<Vec<char>>();
    for ele in d {
        print!("{}", ele);
    }
}

fn to_snafu(a: i64) -> Vec<i64> {
    let mut i = 0;
    loop {
        if (1..=i).map(|x| 5i64.pow(x) * 2).sum::<i64>() - a >= 0 {
            break;
        }
        i += 1;
    }

    let mut k = Vec::<i64>::new();
    let mut n = 0;
    let vv = (-2..=2).collect::<Vec<i64>>();

    for j in (0..=i).rev() {
        let mut m = vv.iter()
            .enumerate()
            .map(|(i, x)| (i, (a - (n + 5i64.pow(j) * x)).abs()))
            .collect::<Vec<(usize, i64)>>();

        m.sort_by_key(|x| x.1);
        
        let b = vv[m.first().unwrap().0];
        k.push(b);
        n += 5i64.pow(j) * b;
    }
    k
}

fn parse<'a>(file: &str, numbers: &'a mut Vec<Vec<i64>>) {
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    for line in buffer.lines().filter(|x| !x.is_empty()) {
        let ns = line.chars().map(|x| {
            match x {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!(),
            }
        }).collect::<Vec<_>>();
        numbers.push(ns);
    }
}