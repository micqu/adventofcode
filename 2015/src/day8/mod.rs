use regex::Regex;

use crate::utils;

pub fn d8() {
    d8_1();
    d8_2();
}

pub fn d8_1() {
    let mut n: Vec<usize> = Vec::new();
    let re = Regex::new(r"\\[^x]|\\[x]..").unwrap();

    let mut reader = utils::Reader::load_input("src/day8/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l = line.unwrap().trim();
        let n_total_chars = l.len();
        
        let mut n_special: usize = 0;
        for cap in re.captures_iter(l) {
            n_special += cap.iter().map(|x| {
                let c = x.unwrap();
                c.end() - c.start() - 1
            }).sum::<usize>();
        }
        let inner = n_total_chars - n_special - 2;
        
        n.push(n_total_chars - inner);
    };
    println!("{}", n.iter().sum::<usize>());
}

pub fn d8_2() {
    let mut n: Vec<usize> = Vec::new();
    let re = Regex::new(r"[^a-z\d\n]").unwrap();

    let mut reader = utils::Reader::load_input("src/day8/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l = line.unwrap().trim();
        let n_total_chars = l.len();
        
        
        let mut n_special: usize = 0;
        for cap in re.captures_iter(l) {
            n_special += cap.iter().count();
        }
        let inner = n_total_chars + n_special + 2;
        n.push(inner - n_total_chars);
    };
    println!("{}", n.iter().sum::<usize>());
}