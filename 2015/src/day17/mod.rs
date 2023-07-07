use crate::utils;

pub fn d17() {
    d17_1();
    d17_2();
}

pub fn d17_1() {
    let mut containers: Vec<Container> = Vec::new();
    parse("src/day17/input.txt", &mut containers);
    
    let mut c = 0;
    for i in 0..(1 << containers.len()) {
        let mut s = 0;
        for j in 0..containers.len() {
            if i & (1 << j) != 0 {
                s += containers[j].vol;
            }
        }
        if s == 150 {
            c += 1;
        }
    }
    println!("{}", c);
}

pub fn d17_2() {
    let mut containers: Vec<Container> = Vec::new();
    parse("src/day17/input.txt", &mut containers);
    
    let mut c = vec![0; containers.len()];
    for i in 0..(1 << containers.len()) {
        let mut s = 0;
        for j in 0..containers.len() {
            if i & (1 << j) != 0 {
                s += containers[j].vol;
            }
        }
        if s == 150 {
            c[(i as i32).count_ones() as usize] += 1;
        }
    }
    let r = c.iter()
        .skip_while(|x| **x == 0)
        .map(|x| *x)
        .collect::<Vec<usize>>();
    println!("{}", r.first().unwrap());
}

fn parse(file: &str, containers: &mut Vec<Container>) {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    buffer.lines().for_each(|line| {
        let c = Container {
            vol: line.trim().parse().unwrap(),
        };
        containers.push(c);
    });
}

#[derive(Debug)]
struct Container {
    vol: u32,
}
