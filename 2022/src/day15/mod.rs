use std::collections::HashSet;

use crate::utils;

pub fn d15() {
    let sensors = parse("src/day15/input.txt");
    // d15_1(&sensors.clone());
    d15_1_but_faster(&sensors.clone(), 2000000);
    d15_2(&sensors);
}

pub fn d15_1_but_faster(sensors: &Vec<Sensor>, row: i64) {
    let mut count = 0;

    let ranges = sensors.iter()
        .filter(|&sensor| row >= sensor.y - sensor.distance() && row <= sensor.y + sensor.distance())
        .map(|sensor| {
            let y_dist = (row - sensor.y).abs();
            let width_at_row = (2 * sensor.distance() + 1) - y_dist * 2;
            count += width_at_row;

            let range = width_at_row / 2;
            (sensor.x - range, sensor.x + range)
        }).collect::<Vec<_>>();

    let mut ranges_uncovered = Vec::<(i64, i64)>::new();
    for i in 0..ranges.len() {
        let a = ranges[i];

        let mut skip = false;
        for j in 0..ranges.len() {
            let b = ranges[j];
            if i == j {
                continue;
            }
            if a.0 >= b.0 && a.1 <= b.1 {
                skip = true;
                count -= a.1 - a.0 + 1;
                break;
            }
        }
        if skip {
            continue;
        }
        ranges_uncovered.push(a);
    }

    for i in 0..ranges_uncovered.len() - 1 {
        let a = ranges_uncovered[i];

        for j in i + 1..ranges_uncovered.len() {
            let b = ranges_uncovered[j];

            if b.0 > a.1 || a.0 > b.1 {
                continue;
            }

            let os = a.0.max(b.0);
            let oe = a.1.min(b.1);
            let overlap = oe - os + 1;
            count -= overlap;
        }
    }
    println!("{}", count - 1);
}

pub fn d15_1(sensors: &Vec<Sensor>) {
    const ROW: i64 = 2000000;
    let mut visited = HashSet::<i64>::new();
    
    sensors.iter()
        .filter(|&sensor| {
            let l1 = sensor.distance();
            sensor.y - l1 <= ROW && ROW <= sensor.y + l1
        })
        .for_each(|sensor| {
            let y_dist = (ROW - sensor.y).abs();
            let width_at_row = (2 * sensor.distance() + 1) - y_dist * 2;
            let range = width_at_row / 2;
            for i in (sensor.x - range)..(sensor.x + range) {
                visited.insert(i);
            }
        });
    println!("{}", visited.len());
}

pub fn d15_2(sensors: &Vec<Sensor>) {
    let limit = 4000000;

    let mut beacon = Beacon { x: 0, y: 0};
    
    for row in 0..=limit {
        let ranges = sensors.iter()
            .filter(|&sensor|
                row >= sensor.y - sensor.distance()
                && row <= sensor.y + sensor.distance()
            )
            .map(|sensor| {
                let y_dist = (row - sensor.y).abs();
                let width_at_row = (2 * sensor.distance() + 1) - y_dist * 2;
                let range = width_at_row / 2;
                (sensor.x - range, sensor.x + range)
            }).collect::<Vec<_>>();
    
        let mut ranges_uncovered = Vec::<(i64, i64)>::new();
        for i in 0..ranges.len() {
            let a = ranges[i];
    
            let mut skip = false;
            for j in 0..ranges.len() {
                let b = ranges[j];
                if i == j {
                    continue;
                }
                if a.0 >= b.0 && a.1 <= b.1 {
                    skip = true;
                    break;
                }
            }
            if skip {
                continue;
            }
            ranges_uncovered.push(a);
        }
    
        let rmin = sensors.iter().map(|sensor| sensor.x - sensor.distance()).min().unwrap();
        let rmax = sensors.iter().map(|sensor| sensor.x + sensor.distance()).max().unwrap();
        ranges_uncovered.push((rmin - 2, rmin - 1));
        ranges_uncovered.push((rmax + 1, rmax + 2));
        ranges_uncovered.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        let mut empty = Vec::<i64>::new();
        for i in 0..ranges_uncovered.len() - 1 {
            let a = ranges_uncovered[i];
            let b = ranges_uncovered[i + 1];

            let xmin = a.1.max(0);
            let xmax = b.0.min(limit);
            if xmax > xmin {
                empty.extend(xmin..xmax + 1);
            }
        }
        
        let free = empty.iter()
            .filter(|&column| sensors.iter()
                .all(|sensor| {
                    let distance_to_empty = (column - sensor.x).abs() + (row - sensor.y).abs();
                    sensor.distance() < distance_to_empty
                })
            ).next();
        
        if let Some(free_x) = free {
            beacon.x = *free_x;
            beacon.y = row;
        }
    }

    let tuning_frequency = beacon.x * 4000000 + beacon.y;
    println!("{}", tuning_frequency);
}

fn parse(file: &str) -> Vec::<Sensor> {
    let mut sensors = Vec::<Sensor>::new();
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    buffer
        .lines()
        .for_each(|line| {
            let tokens = line.split_terminator(&[' ', ',', ':', '='])
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();

            sensors.push(Sensor {
                x: tokens[3].parse().unwrap(),
                y: tokens[5].parse().unwrap(),
                nearest_beacon: Beacon {
                    x: tokens[11].parse().unwrap(),
                    y: tokens[13].parse().unwrap()
                }
            });
        });
    sensors
}

#[derive(Debug, Clone)]
pub struct Sensor {
    pub x: i64,
    pub y: i64,
    pub nearest_beacon: Beacon,
}

impl Sensor {
    pub fn distance(&self) -> i64 {
        (self.nearest_beacon.x - self.x).abs() + (self.nearest_beacon.y - self.y).abs()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
pub struct Beacon {
    pub x: i64,
    pub y: i64,
}