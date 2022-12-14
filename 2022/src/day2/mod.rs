use crate::utils;

pub fn d2_1() {    
    let mut score = 0;
    
    let mut reader = utils::Reader::load_input("src/day2/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l = line.unwrap().trim();
        let split: Vec<&str> = l.split(" ").collect();
        
        let left = split[0].chars().next().unwrap();
        let right = split[1].chars().next().unwrap();

        score += match (left, right) {
            ('A', 'X') => 3,
            ('B', 'Y') => 3,
            ('C', 'Z') => 3,
            ('C', 'X') => 6,
            ('A', 'Y') => 6,
            ('B', 'Z') => 6,
            _ => 0
        };
        
        score += match split[..] {
            [_, "X"] => 1,
            [_, "Y"] => 2,
            [_, "Z"] => 3,
            [] => panic!(),
            [_] => panic!(),
            [_, ..] => panic!(),
        };    
    }

    println!("{}", score);
}

pub fn d2_2() {
    let mut score = 0;

    let mut reader = utils::Reader::load_input("src/day2/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let l = line.unwrap().trim();
        let split: Vec<&str> = l.split(" ").collect();
        
        let left = split[0].chars().next().expect("Should be one char.");
        let right = split[1].chars().next().expect("Should be one char.");

        score += match (left, right) {
            // Even
            ('A', 'Y') => 3,
            ('B', 'Y') => 3,
            ('C', 'Y') => 3,

            // Win
            ('A', 'Z') => 6,
            ('B', 'Z') => 6,
            ('C', 'Z') => 6,
            _ => 0
        };
        
        score += match split[..] {
            // Lose
            ["A", "X"] => 3,
            ["B", "X"] => 1,
            ["C", "X"] => 2,

            // Even
            ["A", "Y"] => 1,
            ["B", "Y"] => 2,
            ["C", "Y"] => 3,

            // Win
            ["A", "Z"] => 2,
            ["B", "Z"] => 3,
            ["C", "Z"] => 1,
            [] => panic!(),
            [_] => panic!(),
            [_, ..] => panic!(),
        };    
    }
    
    println!("{}", score);
}