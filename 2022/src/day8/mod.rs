use crate::utils;

pub fn d8() {
    d8_1();
    d8_2();
}

pub fn d8_1() {
    let mut map = Vec::<Vec<u8>>::new();
    
    let mut reader = utils::Reader::load_input("src/day8/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let trees = line
            .unwrap()
            .trim()
            .as_bytes();

        let mut row: Vec<u8> = Vec::new();
        for t in trees {
            row.push(*t);
        }
        map.push(row);
    }

    let mut count = vec![vec![0 as u32; map.len()]; map.len()];
    for i in 0..map.len() {
        let mut last: u8 = 0;
        for j in 0..map.len() {
            let t = map[i][j];
            if t > last {
                count[i][j] += 1;
                last = t;
            }
        }
        last = 0;
        for j in (0..map.len()).rev() {
            let t = map[i][j];
            if t > last {
                count[i][j] += 1;
                last = t;
            }
        }
    }
    
    for i in 0..map.len() {
        let mut last: u8 = 0;
        for j in 0..map.len() {
            let t = map[j][i];
            if t > last {
                count[j][i] += 1;
                last = t;
            }
        }
        last = 0;
        for j in (0..map.len()).rev() {
            let t = map[j][i];
            if t > last {
                count[j][i] += 1;
                last = t;
            }
        }
    }

    let visible = count.iter().flatten().filter(|x| **x > 0).count();
    println!("{}", visible);
}

pub fn d8_2() {
    let mut map = Vec::<Vec<u8>>::new();
    
    let mut reader = utils::Reader::load_input("src/day8/input.txt").unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let trees = line
            .unwrap()
            .trim()
            .as_bytes();

        let mut row: Vec<u8> = Vec::new();
        for t in trees {
            row.push(*t);
        }
        map.push(row);
    }
    
    let mut count = vec![vec![0 as u32; map.len()]; map.len()];
    
    for i in 0..map.len() {
        for j in 0..map.len() {
            let mut score: u32 = 1;
            let mut amount: u32 = 0;
            for k in j..map.len() {
                if k == j {
                    continue;
                }
                amount += 1;
                if map[i][k] >= map[i][j] {
                    break;
                }
            }
            score *= amount;
            amount = 0;
            for k in (0..j).rev() {
                if k == j {
                    continue;
                }
                amount += 1;
                if map[i][k] >= map[i][j] {
                    break;
                }
            }
            score *= amount;
            amount = 0;
            for k in i..map.len() {
                if k == i {
                    continue;
                }
                amount += 1;
                if map[k][j] >= map[i][j] {
                    break;
                }
            }
            score *= amount;
            amount = 0;
            for k in (0..i).rev() {
                if k == i {
                    continue;
                }
                amount += 1;
                if map[k][j] >= map[i][j] {
                    break;
                }
            }
            score *= amount;
            count[i][j] = score;
        }
    }

    let result = count.iter().flatten().max();
    println!("{}", result.unwrap());
}