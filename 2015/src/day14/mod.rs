use crate::utils;

pub fn d14() {
    d14_1();
    d14_2();
}

pub fn d14_1() {
    let mut reindeer: Vec<Reindeer> = Vec::new();
    parse("src/day14/input.txt", &mut reindeer);
    
    let tt = 2503;
    let r = reindeer.iter().map(|x| {
        (tt / (x.rest + x.flight_time)) * x.spd * x.flight_time
            + (tt % (x.rest + x.flight_time)).min(x.flight_time) * x.spd
    }).max().unwrap();
    println!("{}", r);
}

pub fn d14_2() {
    let mut reindeer: Vec<Reindeer> = Vec::new();
    parse("src/day14/input.txt", &mut reindeer);
    
    let tt = 2503;
    for t in 1..=tt {
        let r: Vec<_> = reindeer.iter().map(|x| {
            (t / (x.rest + x.flight_time)) * x.spd * x.flight_time
            + (t % (x.rest + x.flight_time)).min(x.flight_time) * x.spd
        }).collect();
        
        let m = r.iter().max().unwrap();
        
        r.iter().enumerate().filter(|(_, x)| *x == m).for_each(|(i, _)| {
            reindeer[i].score += 1;
        })
    }
    println!("{}", reindeer.iter().max_by_key(|x| x.score).unwrap().score);
}

fn parse(file: &str, reindeer: &mut Vec<Reindeer>) {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    buffer.lines().for_each(|line| {
        let tokens: Vec<&str> = line.trim().split(' ').collect();
        reindeer.push(Reindeer {
            spd: tokens[3].parse().unwrap(),
            flight_time: tokens[6].parse().unwrap(),
            rest: tokens[13].parse().unwrap(),
            score: 0,
        });
    });
}

#[derive(Debug)]
struct Reindeer {
    spd: u32,
    flight_time: u32,
    rest: u32,
    score: u32,
}