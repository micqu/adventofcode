use std::collections::HashSet;

use crate::utils;

pub fn d19() {
    let mut bps: Vec<Blueprint> = Vec::new();
    parse("src/day19/input.txt", &mut bps);

    d19_1(&bps);
    d19_2(&bps);
}

pub fn d19_1(bps: &Vec<Blueprint>) {
    let mut sum = 0;
    for (i, bp) in bps.iter().enumerate() {
        let start = Visit {
            time: 24,
            resources: [0; 4],
            robots: [1, 0, 0, 0],
        };
        let result = solve(start, bp);
        sum += (i as u32 + 1) * result;
    }
    println!("{}", sum);
}

pub fn d19_2(bps: &Vec<Blueprint>) {    
    let mut sum = 1;
    for bp in bps.iter().take(3) {
        let start = Visit {
            time: 32,
            resources: [0; 4],
            robots: [1, 0, 0, 0],
        };
        let result = solve(start, bp);
        sum *= result;
    }
    println!("{}", sum);
}

fn solve(start: Visit, bp: &Blueprint) -> u32 {
    let mut best_geode_sum = 0;
    let mut visited = HashSet::<Visit>::new();
    let mut q = Vec::<Visit>::new();
    q.push(start);

    while let Some(u) = q.pop() {
        if !visited.insert(u) {
            continue;
        }

        if u.time <= 0 {
            best_geode_sum = best_geode_sum.max(u.resources[3]);
            continue;
        }

        if u.resources[3] > best_geode_sum {
            best_geode_sum = u.resources[3];
        }

        if (u.resources[3] + u.robots[3] * u.time + u.time * (u.time - 1) / 2) < best_geode_sum {
            continue;
        }
        
        let max_resources = [
            bp.robots.iter().map(|x| x.requires[0]).max().unwrap(),
            bp.robots.iter().filter(|x| x.robot_type == RobotType::Obsidian).map(|x| x.requires[1]).max().unwrap(),
            bp.robots.iter().filter(|x| x.robot_type == RobotType::Geode).map(|x| x.requires[1]).max().unwrap(),
        ];
        
        let mut bought = false;
        for bp_robot in bp.robots.iter() {
            let mut new_robots = u.robots.clone();
            let mut new_resources = u.resources.clone();

            let a = bp_robot.requires[0];
            let b = bp_robot.requires[1];

            match bp_robot.robot_type {
                RobotType::Ore => {
                    if u.robots[0] >= max_resources[0] {
                        continue;
                    }

                    if u.resources[0] >= a {
                        new_resources[0] -= a;
                        new_robots[0] += 1;
                        bought = true;
                    }
                },
                RobotType::Clay => {
                    if u.robots[1] >= max_resources[1] {
                        continue;
                    }

                    if u.resources[0] >= a {
                        new_resources[0] -= a;
                        new_robots[1] += 1;
                        bought = true;
                    }
                },
                RobotType::Obsidian => {
                    if u.robots[2] >= max_resources[2] {
                        continue;
                    }

                    if u.resources[0] >= a && u.resources[1] >= b {
                        new_resources[0] -= a;
                        new_resources[1] -= b;
                        new_robots[2] += 1;
                        bought = true;
                    }
                },
                RobotType::Geode => {
                    if u.resources[0] >= a && u.resources[2] >= b {
                        new_resources[0] -= a;
                        new_resources[2] -= b;
                        new_robots[3] += 1;
                        bought = true;
                    }
                },
            }
            
            add_harvest(&mut new_resources, &u.robots);
            q.push(Visit {
                time: u.time - 1,
                resources: new_resources,
                robots: new_robots,
            });
        }
        
        if !bought {
            let mut new_resources = u.resources.clone();
            for i in 1..=u.time {
                add_harvest(&mut new_resources, &u.robots);
                
                let can_buy = bp.robots.iter().any(|x| {
                    match x.robot_type {
                        RobotType::Ore => new_resources[0] >= x.requires[0],
                        RobotType::Clay => new_resources[1] >= x.requires[0],
                        RobotType::Obsidian =>
                            new_resources[0] >= x.requires[0]
                            && new_resources[1] >= x.requires[1],
                        RobotType::Geode =>
                            new_resources[0] >= x.requires[0]
                            && new_resources[2] >= x.requires[1],
                    }
                });
                
                if can_buy {
                    q.push(Visit {
                        time: u.time - i,
                        resources: new_resources,
                        robots: u.robots,
                    });
                    break;
                }
            }
        }
    }
    best_geode_sum
}

fn add_harvest(resources: &mut [u32; 4], robots: &[u32; 4]) {
    for i in 0..4 {
        resources[i] += robots[i];
    }
}

fn parse<'a>(file: &str, blueprints: &'a mut Vec<Blueprint>) {
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    buffer.lines()
        .enumerate()
        .for_each(|(i, x)| {
            let tokens = x.split(' ').collect::<Vec<&str>>();
            let robot_ore = Robot {
                robot_type: RobotType::Ore,
                requires: [tokens[6].parse().unwrap(), 0]
            };
            let robot_cla = Robot {
                robot_type: RobotType::Clay,
                requires: [tokens[12].parse::<u32>().unwrap(), 0]
            };
            let robot_obs = Robot {
                robot_type: RobotType::Obsidian,
                requires: [
                    tokens[18].parse::<u32>().unwrap(),
                    tokens[21].parse::<u32>().unwrap()
                ]
            };
            let robot_geo = Robot {
                robot_type: RobotType::Geode,
                requires: [
                    tokens[27].parse::<u32>().unwrap(),
                    tokens[30].parse::<u32>().unwrap()
                ]
            };
            let robots = vec![robot_ore, robot_cla, robot_geo, robot_obs];
            let bp = Blueprint { id: i + 1, robots: robots };
            blueprints.push(bp);
        });
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Visit {
    time: u32,
    resources: [u32; 4],
    robots: [u32; 4],
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Blueprint {
    id: usize,
    robots: Vec<Robot>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Robot {
    robot_type: RobotType,
    requires: [u32; 2],
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}