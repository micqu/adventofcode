use crate::utils;

pub fn d23() {
    d23_1();
    d23_2();
}

pub fn d23_1() {
    let instructions = parse("src/day23/input.txt");
    let mut reg = vec![0, 0];
    let mut i: i32 = 0;
    while i >= 0 && (i as usize) < instructions.len() {
        match instructions[i as usize] {
            Instruction::Hlf(r) => reg[r as usize] /= 2,
            Instruction::Tpl(r) => reg[r as usize] *= 3,
            Instruction::Inc(r) => reg[r as usize] += 1,
            Instruction::Jmp(off) => i += off - 1,
            Instruction::Jie(r, off) => if reg[r as usize] % 2 == 0 { i += off - 1; },
            Instruction::Jio(r, off) => if reg[r as usize] == 1 { i += off - 1; },
        }
        i += 1;
    }
    println!("{}", reg[1]);
}

pub fn d23_2() {
    let instructions = parse("src/day23/input.txt");
    let mut reg = vec![1, 0];
    let mut i: i32 = 0;
    while i >= 0 && (i as usize) < instructions.len() {
        match instructions[i as usize] {
            Instruction::Hlf(r) => reg[r as usize] /= 2,
            Instruction::Tpl(r) => reg[r as usize] *= 3,
            Instruction::Inc(r) => reg[r as usize] += 1,
            Instruction::Jmp(off) => i += off - 1,
            Instruction::Jie(r, off) => if reg[r as usize] % 2 == 0 { i += off - 1; },
            Instruction::Jio(r, off) => if reg[r as usize] == 1 { i += off - 1; },
        }
        i += 1;
    }
    println!("{}", reg[1]);
}

fn parse(file: &str) -> Vec<Instruction> {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    reader.read(&mut buffer);
    let mut instructions = Vec::<Instruction>::new();

    for line in buffer.lines() {
        let tokens: Vec<_> = line.trim().split(&[' ', ',', '+']).filter(|x| !x.is_empty()).collect();
        match tokens[0] {
            "hlf" => {
                instructions.push(Instruction::Hlf(char_to_int(tokens[1].chars().next().unwrap())));
            },
            "tpl" => {
                instructions.push(Instruction::Tpl(char_to_int(tokens[1].chars().next().unwrap())));
            },
            "inc" => {
                instructions.push(Instruction::Inc(char_to_int(tokens[1].chars().next().unwrap())));
            },
            "jmp" => {
                let a = tokens[1].parse::<i32>().unwrap();
                instructions.push(Instruction::Jmp(a));
            },
            "jie" => {
                let b = tokens[2].parse().unwrap();
                instructions.push(Instruction::Jie(char_to_int(tokens[1].chars().next().unwrap()), b));
            },
            "jio" => {
                let b = tokens[2].parse().unwrap();
                instructions.push(Instruction::Jio(char_to_int(tokens[1].chars().next().unwrap()), b));
            },
            _ => panic!(),
        };
    }
    instructions
}

fn char_to_int(ch: char) -> u32 {
    ch as u32 - 'a' as u32
}

#[derive(Debug)]
enum Instruction {
    Hlf(u32),
    Tpl(u32),
    Inc(u32),
    Jmp(i32),
    Jie(u32, i32),
    Jio(u32, i32),
}