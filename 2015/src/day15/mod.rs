use crate::utils;

pub fn d15() {
    d15_1();
    d15_2();
}

pub fn d15_1() {
    let mut ings: Vec<Ingredient> = Vec::new();
    parse("src/day15/input.txt", &mut ings);

    let l = ings.len() - 1;
    let mut m = 0;
    for a in 0..100 - l {
        for b in a + 1..100 - l {
            for c in b + 1..100 - l {
                let s1 = ings[0].get_score(a as i64);
                let s2 = ings[1].get_score((b - a) as i64);
                let s3 = ings[2].get_score((c - b) as i64);
                let s4 = ings[3].get_score((100 - c) as i64);
                let p = (0..4)
                    .map(|i| (s1[i] + s2[i] + s3[i] + s4[i]).max(0))
                    .product();
                m = m.max(p);
            }
        }
    }
    println!("{:?}", m);
}

pub fn d15_2() {
    let mut ings: Vec<Ingredient> = Vec::new();
    parse("src/day15/input.txt", &mut ings);

    let l = ings.len() - 1;
    let mut m = 0;
    for a in 0..100 - l {
        for b in a + 1..100 - l {
            for c in b + 1..100 - l {
                let s1 = ings[0].get_score(a as i64);
                let s2 = ings[1].get_score((b - a) as i64);
                let s3 = ings[2].get_score((c - b) as i64);
                let s4 = ings[3].get_score((100 - c) as i64);

                if s1[4] + s2[4] + s3[4] + s4[4] == 500 {
                    let p = (0..4)
                        .map(|i| (s1[i] + s2[i] + s3[i] + s4[i]).max(0))
                        .product();
                    m = m.max(p);
                }
            }
        }
    }
    println!("{:?}", m);
}

fn parse(file: &str, ingredients: &mut Vec<Ingredient>) {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    buffer.lines().for_each(|line| {
        let tokens: Vec<&str> = line.trim().split(&[' ', ',']).collect();
        ingredients.push(Ingredient {
            capacity: tokens[2].parse().unwrap(),
            durability: tokens[5].parse().unwrap(),
            flavor: tokens[8].parse().unwrap(),
            texture: tokens[11].parse().unwrap(),
            calories: tokens[14].parse().unwrap(),
        });
    });
}

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn get_score(&self, amount: i64) -> [i64; 5] {
        [
            self.capacity * amount,
            self.durability * amount,
            self.flavor * amount,
            self.texture * amount,
            self.calories * amount,
        ]
    }
}
