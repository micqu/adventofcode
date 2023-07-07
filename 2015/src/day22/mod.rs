use std::collections::BinaryHeap;

use crate::utils;

pub fn d22() {
    d22_1();
    d22_2();
}

pub fn d22_1() {
    let boss = parse("src/day22/input.txt");
    let me = Entity { hp: 50, dmg: 0, armor: 0, mana: 500 };
    let start = State { turn: 0, cost: 0, me, boss, effects: Vec::new() };

    let effects = vec![
        Effect { cost: 53, begin: 0, duration: 0, effect_type: EffectType::MagicMissile(4) },
        Effect { cost: 73, begin: 0, duration: 0, effect_type: EffectType::Drain(2, 2) },
        Effect { cost: 113, begin: 0, duration: 6, effect_type: EffectType::Shield(7) },
        Effect { cost: 173, begin: 0, duration: 6, effect_type: EffectType::Poison(3) },
        Effect { cost: 229, begin: 0, duration: 5, effect_type: EffectType::Recharge(101) },
    ];

    let cost = solve(start, &effects).unwrap();
    println!("{}", cost);
}

fn solve(start: State, effects: &Vec<Effect>) -> Option<i32> {
    let mut q = BinaryHeap::<State>::new();
    q.push(start);

    while let Some(mut u) = q.pop() {
        let turn = u.turn + 1;
        
        for effect in u.effects.iter() {
            match effect.effect_type {
                EffectType::Shield(armor) => u.me.armor = armor,
                EffectType::Poison(dmg) => u.boss.hp -= dmg,
                EffectType::Recharge(mana) => u.me.mana += mana,
                _ => { },
            }
            if effect.begin + effect.duration - turn <= 0 {
                match effect.effect_type {
                    EffectType::Shield(_) => u.me.armor = 0,
                    _ => { },
                }
            }
        }

        if u.boss.hp <= 0 {
            return Some(u.cost);
        }

        if turn % 2  == 0 {
            u.me.hp -= u.me.get_mitigated_dmg(u.boss.dmg);

            if u.me.hp > 0 {
                q.push(State {
                    turn,
                    cost: u.cost,
                    me: u.me,
                    boss: u.boss,
                    effects: u.effects.iter().filter(|&x| x.begin + x.duration - turn > 0).map(|x| x.clone()).collect(),
                });
            }
            
            continue;
        }

        for effect in effects.iter() {
            if u.effects.iter().any(|x|
                x.effect_type == effect.effect_type
                && x.begin + x.duration - turn > 0
            ) {
                continue;
            }

            let mut me = u.me.clone();
            me.mana -= effect.cost;
            if me.mana < 0 {
                continue;
            }

            let mut boss = u.boss.clone();

            if effect.duration == 0 {
                match effect.effect_type {
                    EffectType::MagicMissile(dmg) => boss.hp -= dmg,
                    EffectType::Drain(dmg, heal) => {
                        boss.hp -= dmg;
                        me.hp += heal;
                    },
                    _ => { },
                }
                
                if boss.hp <= 0 {
                    return Some(u.cost + effect.cost);
                }

                q.push(State {
                    turn,
                    cost: u.cost + effect.cost,
                    me: me,
                    boss: boss,
                    effects: u.effects.iter().filter(|&x| x.begin + x.duration - turn > 0).map(|x| x.clone()).collect(),
                });
            } else {
                let mut nes: Vec<Effect> = u.effects.iter().filter(|&x| x.begin + x.duration - turn > 0).map(|x| x.clone()).collect();
                let mut ne = effect.clone();
                ne.begin = turn;
                nes.push(ne);
    
                q.push(State {
                    turn,
                    cost: u.cost + effect.cost,
                    me: me,
                    boss: boss,
                    effects: nes,
                });
            }
        }
    }
    None
}

pub fn d22_2() {
    let boss = parse("src/day22/input.txt");
    let me = Entity { hp: 50, dmg: 0, armor: 0, mana: 500 };
    let start = State { turn: 0, cost: 0, me, boss, effects: Vec::new() };

    let effects = vec![
        Effect { cost: 53, begin: 0, duration: 0, effect_type: EffectType::MagicMissile(4) },
        Effect { cost: 73, begin: 0, duration: 0, effect_type: EffectType::Drain(2, 2) },
        Effect { cost: 113, begin: 0, duration: 6, effect_type: EffectType::Shield(7) },
        Effect { cost: 173, begin: 0, duration: 6, effect_type: EffectType::Poison(3) },
        Effect { cost: 229, begin: 0, duration: 5, effect_type: EffectType::Recharge(101) },
    ];

    let cost = solve2(start, &effects).unwrap();
    println!("{}", cost);
}

fn solve2(start: State, effects: &Vec<Effect>) -> Option<i32> {
    let mut q = BinaryHeap::<State>::new();
    q.push(start);

    while let Some(mut u) = q.pop() {
        let turn = u.turn + 1;
        
        if (turn + 1) % 2 == 0 {
            u.me.hp -= 1;
            if u.me.hp <= 0 {
                continue;
            }
        }

        for effect in u.effects.iter() {
            match effect.effect_type {
                EffectType::Shield(armor) => u.me.armor = armor,
                EffectType::Poison(dmg) => u.boss.hp -= dmg,
                EffectType::Recharge(mana) => u.me.mana += mana,
                _ => { },
            }
            if effect.begin + effect.duration - turn <= 0 {
                match effect.effect_type {
                    EffectType::Shield(_) => u.me.armor = 0,
                    _ => { },
                }
            }
        }

        if u.boss.hp <= 0 {
            return Some(u.cost);
        }

        if turn % 2  == 0 {
            u.me.hp -= u.me.get_mitigated_dmg(u.boss.dmg);

            if u.me.hp > 0 {
                q.push(State {
                    turn,
                    cost: u.cost,
                    me: u.me,
                    boss: u.boss,
                    effects: u.effects.iter().filter(|&x| x.begin + x.duration - turn > 0).map(|x| x.clone()).collect(),
                });
            }
            
            continue;
        }

        for effect in effects.iter() {
            if u.effects.iter().any(|x|
                x.effect_type == effect.effect_type
                && x.begin + x.duration - turn > 0
            ) {
                continue;
            }

            let mut me = u.me.clone();
            me.mana -= effect.cost;
            if me.mana < 0 {
                continue;
            }

            let mut boss = u.boss.clone();

            if effect.duration == 0 {
                match effect.effect_type {
                    EffectType::MagicMissile(dmg) => boss.hp -= dmg,
                    EffectType::Drain(dmg, heal) => {
                        boss.hp -= dmg;
                        me.hp += heal;
                    },
                    _ => { },
                }
                
                if boss.hp <= 0 {
                    return Some(u.cost + effect.cost);
                }

                q.push(State {
                    turn,
                    cost: u.cost + effect.cost,
                    me: me,
                    boss: boss,
                    effects: u.effects.iter().filter(|&x| x.begin + x.duration - turn > 0).map(|x| x.clone()).collect(),
                });
            } else {
                let mut nes: Vec<Effect> = u.effects.iter().filter(|&x| x.begin + x.duration - turn > 0).map(|x| x.clone()).collect();
                let mut ne = effect.clone();
                ne.begin = turn;
                nes.push(ne);
    
                q.push(State {
                    turn,
                    cost: u.cost + effect.cost,
                    me: me,
                    boss: boss,
                    effects: nes,
                });
            }
        }
    }
    None
}

#[derive(Debug)]
struct State {
    turn: i32,
    cost: i32,
    me: Entity,
    boss: Entity,
    effects: Vec<Effect>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
            && self.me == other.me
            && self.boss == other.boss
            && self.effects.iter().all(|a| other.effects.iter().any(|b| *a == *b))
    }
}

fn parse(file: &str) -> Entity {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    let mut boss = Entity::default();
    for line in buffer.lines() {
        let tokens: Vec<_> = line.trim().split(&[' ', ':']).collect();
        match tokens[0] {
            "Hit" => boss.hp = tokens.last().unwrap().parse().unwrap(),
            "Damage" => boss.dmg = tokens.last().unwrap().parse().unwrap(),
            _ => panic!(),
        };
    }
    boss
}

#[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Entity {
    hp: i32,
    dmg: i32,
    armor: i32,
    mana: i32,
}

impl Entity {
    fn get_mitigated_dmg(&self, dmg: i32) -> i32 {
        (dmg - self.armor).max(1)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Effect {
    cost: i32,
    begin: i32,
    duration: i32,
    effect_type: EffectType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum EffectType {
    MagicMissile(i32),
    Drain(i32, i32),
    Shield(i32),
    Poison(i32),
    Recharge(i32),
}