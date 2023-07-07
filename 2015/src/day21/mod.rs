use crate::utils;

pub fn d21() {
    d21_1();
    d21_2();
}

pub fn d21_1() {
    let boss = parse("src/day21/input.txt");
    let items = vec![
        Item { id: 0, cost: 8, dmg: 4, armor: 0, item_type: ItemType::Weapon },
        Item { id: 1, cost: 10, dmg: 5, armor: 0, item_type: ItemType::Weapon },
        Item { id: 2, cost: 25, dmg: 6, armor: 0, item_type: ItemType::Weapon },
        Item { id: 3, cost: 40, dmg: 7, armor: 0, item_type: ItemType::Weapon },
        Item { id: 4, cost: 74, dmg: 8, armor: 0, item_type: ItemType::Weapon },

        Item { id: 5, cost: 13, dmg: 0, armor: 1, item_type: ItemType::Armor },
        Item { id: 6, cost: 31, dmg: 0, armor: 2, item_type: ItemType::Armor },
        Item { id: 7, cost: 53, dmg: 0, armor: 3, item_type: ItemType::Armor },
        Item { id: 8, cost: 75, dmg: 0, armor: 4, item_type: ItemType::Armor },
        Item { id: 9, cost: 102, dmg: 0, armor: 5, item_type: ItemType::Armor },

        Item { id: 10, cost: 25, dmg: 1, armor: 0, item_type: ItemType::Ring },
        Item { id: 11, cost: 50, dmg: 2, armor: 0, item_type: ItemType::Ring },
        Item { id: 12, cost: 100, dmg: 3, armor: 0, item_type: ItemType::Ring },
        Item { id: 14, cost: 20, dmg: 0, armor: 1, item_type: ItemType::Ring },
        Item { id: 15, cost: 40, dmg: 0, armor: 2, item_type: ItemType::Ring },
        Item { id: 16, cost: 80, dmg: 0, armor: 3, item_type: ItemType::Ring },
    ];

    let mut rings: Vec<(Option<u32>, Option<u32>)> = vec![(None, None)];
    for i in 0..6 {
        rings.push((Some(i as u32 + 10), None));
    }
    for i in 0..6 - 1 {
        for j in i + 1..6 {
            let a = Some(i as u32 + 10);
            let b = Some(j as u32 + 10);
            rings.push((a, b));
        }
    }
    let weapon = (0..=4 as u32).collect::<Vec<_>>();
    let mut armor = (5..=9 as u32).map(|x| Some(x)).collect::<Vec<_>>();
    armor.push(None);

    let mut c_min = i32::MAX;
    let mut c_max = i32::MIN;
    for w in weapon.iter() {
        for a in armor.iter() {
            for (r1, r2) in rings.iter() {
                let mut c = 0;
                
                let mut me = Entity { hp: 100, dmg: 0, armor: 0 };
                let i1 = &items[*w as usize];
                c += i1.cost;
                me.add_item(i1);

                if let Some(l) = a {
                    let i2 = &items[*l as usize];
                    c += i2.cost;
                    me.add_item(i2);
                }

                if let Some(l) = r1 {
                    let i3 = &items[*l as usize];
                    c += i3.cost;
                    me.add_item(i3);
                }
                if let Some(r) = r2 {
                    let i4 = &items[*r as usize];
                    c += i4.cost;
                    me.add_item(i4);
                }

                let mdmg = boss.get_mitigated_dmg(me.dmg);
                let mtd = boss.hp / mdmg;
                
                let bdmg = me.get_mitigated_dmg(boss.dmg);
                let btd = me.hp / bdmg;
            
                if mtd <= btd {
                    c_min = c_min.min(c);
                } else {
                    c_max = c_max.max(c);
                }
            }
        }
    }
    println!("{}", c_min);
    println!("{}", c_max);
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct State {
    gold: u32,
    me: Entity,
    boss: Entity,
}

pub fn d21_2() {}

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
            "Armor" => boss.armor = tokens.last().unwrap().parse().unwrap(),
            _ => panic!(),
        };
    }
    boss
}

#[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct Entity {
    hp: i32,
    dmg: i32,
    armor: i32,
}

impl Entity {
    fn get_mitigated_dmg(&self, dmg: i32) -> i32 {
        (dmg - self.armor).max(1)
    }

    fn add_item(&mut self, item: &Item) {
        self.dmg += item.dmg;
        self.armor += item.armor;
    }
}

#[derive(Debug)]
struct Item {
    id: u32,
    cost: i32,
    dmg: i32,
    armor: i32,
    item_type: ItemType,
}

#[derive(Debug)]
enum ItemType {
    Weapon,
    Armor,
    Ring,
}