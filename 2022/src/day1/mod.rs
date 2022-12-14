use crate::utils;

pub fn d1() {
    d1_1();
    d1_2();
}

pub fn d1_1() {
    let mut curr = 0;
    let mut sum = 0;

    let mut reader = utils::Reader::load_input("src/day1/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l = line.unwrap().trim();

        if !l.is_empty() {
            let n: i32 = l.parse().unwrap();
            curr += n;
            if curr > sum {
                sum = curr;
            }
            continue;
        }
        curr = 0;
    }
    println!("{}", sum);
}

pub fn d1_2() {    
    let mut v: Vec<i32> = Vec::new();
    let mut curr = 0;

    let mut reader = utils::Reader::load_input("src/day1/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l = line.unwrap().trim();

        if !l.is_empty() {
            let n: i32 = l.parse().unwrap();
            curr += n;
            continue;
        }
        v.push(curr);
        curr = 0;
    }
    v.push(curr);

    v.sort_by(|a, b| b.cmp(a));
    let total: i32 = v[..3].iter().sum();
    println!("{}", total);
}