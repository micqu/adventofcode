use std::collections::{HashMap, HashSet};

use crate::utils;

pub fn d7() {
    d7_1();
    d7_2();
}

pub fn d7_1() {
    let mut signals = HashMap::<String, Signal>::new();
    let mut tmp = HashMap::<String, Signal>::new();
    let mut checked = HashSet::<String>::new();
    parse("src/day7/input.txt", &mut signals);

    loop {
        tmp.clear();

        for (id, sig) in signals.iter() {
            if checked.contains(id) {
                continue;
            }
            if let Some(v) = sig.val {
                checked.insert(id.to_string());

                for (id2, sig) in signals.iter() {
                    if let Some(op) = &sig.op {
                        let mut nk = sig.clone();
                        nk.op = Some(set_op(id, v, &op));
                        set_val(&mut nk);
                        tmp.insert(id2.clone(), nk);
                    }
                }
                break;
            }
        }
        
        signals.extend(tmp.clone());
        if tmp.len() == 0 {
            break;
        }
    }
    println!("{:?}", signals["a"].val);
}

pub fn d7_2() {
    let mut signals = HashMap::<String, Signal>::new();
    let mut tmp = HashMap::<String, Signal>::new();
    let mut checked = HashSet::<String>::new();
    parse("src/day7/input.txt", &mut signals);

    let b = signals.get_mut("b").unwrap();
    b.val = Some(46065);
    
    loop {
        tmp.clear();

        for (id, sig) in signals.iter() {
            if checked.contains(id) {
                continue;
            }
            if let Some(v) = sig.val {
                checked.insert(id.to_string());

                for (id2, sig) in signals.iter() {
                    if let Some(op) = &sig.op {
                        let mut nk = sig.clone();
                        nk.op = Some(set_op(id, v, &op));
                        set_val(&mut nk);
                        tmp.insert(id2.clone(), nk);
                    }
                }
                break;
            }
        }
        
        signals.extend(tmp.clone());
        if tmp.len() == 0 {
            break;
        }
    }
    println!("{:?}", signals["a"].val);
}

fn set_val(signal: &mut Signal) {
    if let Some(o) = &signal.op {
        match o {
            Op::And(a, b) => {
                if let (Operand::Val(a), Operand::Val(b)) = (a, b) {
                    signal.val = Some(a & b);
                }
            },
            Op::Or(a, b) => {
                if let (Operand::Val(a), Operand::Val(b)) = (a, b) {
                    signal.val = Some(a | b);
                }
            },
            Op::Lshift(a, b) => {
                if let (Operand::Val(a), Operand::Val(b)) = (a, b) {
                    signal.val = Some(a << b);
                }
            },
            Op::Rshift(a, b) => {
                if let (Operand::Val(a), Operand::Val(b)) = (a, b) {
                    signal.val = Some(a >> b);
                }
            },
            Op::Not(a) => {
                if let Operand::Val(a) = a {
                    signal.val = Some(!a);
                }
            },
            Op::Pass(a) => {
                if let Operand::Val(a) = a {
                    signal.val = Some(*a);
                }
            },
        }
    }
}

fn set_op(id: &String, val: u16, op: &Op) -> Op {
    match op {
        Op::And(a, b) => {
            let (l, r) = set_operands(id, val, a, b);
            return Op::And(l, r);
        },
        Op::Or(a, b) => {
            let (l, r) = set_operands(id, val, a, b);
            return Op::Or(l, r);
        },
        Op::Lshift(a, b) => {
            let (l, r) = set_operands(id, val, a, b);
            return Op::Lshift(l, r);
        },
        Op::Rshift(a, b) => {
            let (l, r) = set_operands(id, val, a, b);
            return Op::Rshift(l, r);
        },
        Op::Not(a) => {
            let l = set_operand(id, val, a);
            return Op::Not(l);
        },
        Op::Pass(a) => {
            let l = set_operand(id, val, a);
            return Op::Pass(l);
        }
    }
}

fn set_operands(id: &String, val: u16, a: &Operand, b: &Operand) -> (Operand, Operand) {
    (set_operand(id, val, a), set_operand(id, val, b))
}

fn set_operand(id: &String, val: u16, a: &Operand) -> Operand {
    match a {
        Operand::Var(v) if *v == *id => Operand::Val(val),
        _ => a.clone(),
    }
}

fn parse(file: &str, signals: &mut HashMap<String, Signal>) {
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let split: Vec<&str> = line.unwrap().trim().split(" -> ").collect();

        if let Ok(n) = split[0].parse::<u16>() {
            let sig = Signal {
                id: split[1].to_string(),
                val: Some(n),
                op: None,
            };
            signals.insert(sig.id.clone(), sig);
        } else {
            let inp: Vec<&str> = split[0].split_whitespace().collect();

            if inp.len() == 1 {
                let sig = Signal {
                    id: split[1].to_string(),
                    val: None,
                    op: Some(Op::Pass(parse_operand(inp[0]))),
                };
                signals.insert(sig.id.clone(), sig);
            } else {
                if inp[0] == "NOT" {
                    let sig = Signal {
                        id: split[1].to_string(),
                        val: None,
                        op: Some(Op::Not(parse_operand(inp[1]))),
                    };
                    signals.insert(sig.id.clone(), sig);
                } else {
                    let sig = Signal {
                        id: split[1].to_string(),
                        val: None,
                        op: match inp[1] {
                            "AND" => Some(Op::And(parse_operand(inp[0]), parse_operand(inp[2]))),
                            "OR" => Some(Op::Or(parse_operand(inp[0]), parse_operand(inp[2]))),
                            "LSHIFT" => Some(Op::Lshift(parse_operand(inp[0]), parse_operand(inp[2]))),
                            "RSHIFT" => Some(Op::Rshift(parse_operand(inp[0]), parse_operand(inp[2]))),
                            _ => panic!()
                        }
                    };
                    signals.insert(sig.id.clone(), sig);
                }
            }
        }
    };
}

fn parse_operand(operand: &str) -> Operand {
    if let Ok(n) = operand.parse::<u16>() {
        return Operand::Val(n);
    }
    Operand::Var(operand.to_string())
}

#[derive(Debug, Clone)]
struct Signal {
    id: String,
    val: Option<u16>,
    op: Option<Op>,
}

#[derive(Debug, Clone)]
enum Op {
    And(Operand, Operand),
    Or(Operand, Operand),
    Lshift(Operand, Operand),
    Rshift(Operand, Operand),
    Not(Operand),
    Pass(Operand),
}

#[derive(Debug, Clone)]
enum Operand {
    Val(u16),
    Var(String),
}