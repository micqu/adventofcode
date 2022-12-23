use core::panic;
use std::collections::HashMap;

use crate::utils;

pub fn d21() {
    d21_1();
    d21_2();
}

pub fn d21_1() {
    let mut monkeys = Vec::<Monkey>::new();
    let edges = parse("src/day21/input.txt", &mut monkeys);
    let root = edges.keys().find(|x| x.name == "root").unwrap();
    let result = eval(&root, &edges);
    println!("{}", result);
}

pub fn d21_2() {
    let mut monkeys = Vec::<Monkey>::new();
    let mut edges = parse("src/day21/input.txt", &mut monkeys);
    
    let root = edges.keys().find(|x| x.name == "root").unwrap().clone();
    let mut new_root = root.clone();
    new_root.monkey_type = MonkeyType::Eq;
    let k = edges.get(root).unwrap().clone();
    edges.insert(&new_root, k);
    
    let tree = create_monkey_tree(&new_root, &edges);
    let result = solve(&tree, 1);
    println!("{}", result);
}

fn solve(node: &MonkeyNode, val: u64) -> u64 {
    match node.monkey_type {
        MonkeyType::Number(_) => {
            if node.val.is_none() && node.monkey_type == MonkeyType::Number(0) {
                return val;
            }
            return node.val.unwrap();
        },
        MonkeyType::Add => {
            if let Some(cs) = &node.children {
                if let Some(l) = &cs[0].val {
                    return solve(&cs[1], val - l);
                }
                if let Some(r) = &cs[1].val {
                    return solve(&cs[0], val - r);
                }
            }
            panic!();
        },
        MonkeyType::Sub => {
            if let Some(cs) = &node.children {
                if let Some(l) = &cs[0].val {
                    return solve(&cs[1], l - val);
                }
                if let Some(r) = &cs[1].val {
                    return solve(&cs[0], val + r);
                }
            }
            panic!();
        },
        MonkeyType::Mul => {
            if let Some(cs) = &node.children {
                if let Some(l) = &cs[0].val {
                    return solve(&cs[1], val / l);
                }
                if let Some(r) = &cs[1].val {
                    return solve(&cs[0], val / r);
                }
            }
            panic!();
        },
        MonkeyType::Div => {
            if let Some(cs) = &node.children {
                if let Some(l) = &cs[0].val {
                    return solve(&cs[1], l / val);
                }
                if let Some(r) = &cs[1].val {
                    return solve(&cs[0], val * r);
                }
            }
            panic!();
        },
        MonkeyType::Eq => {
            if let Some(cs) = &node.children {
                if let Some(l) = &cs[0].val {
                    return solve(&cs[1], *l);
                }
                if let Some(r) = &cs[1].val {
                    return solve(&cs[0], *r);
                }
            }
            panic!();
        },
    }
}

fn create_monkey_tree(root: &Monkey, edges: &HashMap<&Monkey, Vec<&Monkey>>) -> MonkeyNode {
    if root.name == "humn" {
        return MonkeyNode {
            val: None,
            children: None,
            monkey_type: MonkeyType::Number(0),
        };
    }

    match root.monkey_type {
        MonkeyType::Number(n) => {
            MonkeyNode {
                val: Some(n),
                children: None,
                monkey_type: MonkeyType::Number(n),
            }
        },
        MonkeyType::Add => {
            let a = create_monkey_tree(&edges[root][0], edges);
            let b = create_monkey_tree(&edges[root][1], edges);
            if a.val.is_some() && b.val.is_some() {
                let result = a.val.unwrap() + b.val.unwrap();
                return MonkeyNode {
                    val: Some(result),
                    children: None,
                    monkey_type: MonkeyType::Number(result),
                };
            }
            MonkeyNode {
                val: None,
                children: Some(vec![a, b]),
                monkey_type: MonkeyType::Add,
            }
        },
        MonkeyType::Sub => {
            let a = create_monkey_tree(&edges[root][0], edges);
            let b = create_monkey_tree(&edges[root][1], edges);
            if a.val.is_some() && b.val.is_some() {
                let result = a.val.unwrap() - b.val.unwrap();
                return MonkeyNode {
                    val: Some(result),
                    children: None,
                    monkey_type: MonkeyType::Number(result),
                };
            }
            MonkeyNode {
                val: None,
                children: Some(vec![a, b]),
                monkey_type: MonkeyType::Sub,
            }
        },
        MonkeyType::Mul => {
            let a = create_monkey_tree(&edges[root][0], edges);
            let b = create_monkey_tree(&edges[root][1], edges);
            if a.val.is_some() && b.val.is_some() {
                let result = a.val.unwrap() * b.val.unwrap();
                return MonkeyNode {
                    val: Some(result),
                    children: None,
                    monkey_type: MonkeyType::Number(result),
                };
            }
            MonkeyNode {
                val: None,
                children: Some(vec![a, b]),
                monkey_type: MonkeyType::Mul,
            }
        },
        MonkeyType::Div => {
            let a = create_monkey_tree(&edges[root][0], edges);
            let b = create_monkey_tree(&edges[root][1], edges);
            if a.val.is_some() && b.val.is_some() {
                let result = a.val.unwrap() / b.val.unwrap();
                return MonkeyNode {
                    val: Some(result),
                    children: None,
                    monkey_type: MonkeyType::Number(result),
                };
            }
            MonkeyNode {
                val: None,
                children: Some(vec![a, b]),
                monkey_type: MonkeyType::Div,
            }
        },
        MonkeyType::Eq => {
            let a = create_monkey_tree(&edges[root][0], edges);
            let b = create_monkey_tree(&edges[root][1], edges);
            if a.val.is_some() && b.val.is_some() {
                let result = (a.val.unwrap() == b.val.unwrap()) as u64;
                return MonkeyNode {
                    val: Some(result),
                    children: None,
                    monkey_type: MonkeyType::Number(result),
                };
            }
            MonkeyNode {
                val: None,
                children: Some(vec![a, b]),
                monkey_type: MonkeyType::Eq,
            }
        },
    }
}

fn eval(root: &Monkey, edges: &HashMap<&Monkey, Vec<&Monkey>>) -> u64 {
    match root.monkey_type {
        MonkeyType::Number(n) => {
            return n;
        },
        MonkeyType::Add => {
            let ops = &edges[root];
            return eval(ops[0], edges) + eval(ops[1], edges);
        },
        MonkeyType::Sub => {
            let ops = &edges[root];
            return eval(ops[0], edges) - eval(ops[1], edges);
        },
        MonkeyType::Mul => {
            let ops = &edges[root];
            return eval(ops[0], edges) * eval(ops[1], edges);
        },
        MonkeyType::Div => {
            let ops = &edges[root];
            return eval(ops[0], edges) / eval(ops[1], edges);
        },
        MonkeyType::Eq => {
            let ops = &edges[root];
            return (eval(ops[0], edges) == eval(ops[1], edges)) as u64;
        },
    }
}

fn parse<'a>(file: &str, monkeys: &'a mut Vec<Monkey>) -> HashMap::<&'a Monkey, Vec<&'a Monkey>> {
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();

    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    buffer.lines()
        .for_each(|x| {
            let tokens = x.split(&[':', ' ']).collect::<Vec<&str>>();
            match tokens.len() {
                5 => {
                    let name = tokens[0].to_owned();
                    let entry = edges.entry(name.clone()).or_insert(Vec::new());
                    entry.push(tokens[2].to_owned());
                    entry.push(tokens[3].to_owned());
                    entry.push(tokens[4].to_owned());
                    
                    let m = Monkey {
                        name: name.clone(),
                        monkey_type: match tokens[3] {
                            "+" => MonkeyType::Add,
                            "-" => MonkeyType::Sub,
                            "*" => MonkeyType::Mul,
                            "/" => MonkeyType::Div,
                            _ => panic!(),
                        },
                    };
                    monkeys.push(m);
                },
                3 => {
                    let name = tokens[0].to_owned();
                    let entry = edges.entry(name.clone()).or_insert(Vec::new());
                    entry.push(tokens[2].to_owned());

                    let number = tokens[2].parse::<u64>().unwrap();
                    let m = Monkey {
                        name: name.clone(),
                        monkey_type: MonkeyType::Number(number),
                    };
                    monkeys.push(m);
                }
                _ => panic!(),
            }
        });

    let mut neighbours = HashMap::<&Monkey, Vec<&Monkey>>::new();
    for monkey in monkeys.iter() {
        if let MonkeyType::Number(_) = monkey.monkey_type {
            continue;
        }

        let ed = edges.get(&monkey.name).unwrap();
        let n = monkeys.iter().find(|x| x.name == ed[0]).unwrap();
        neighbours.entry(&monkey).or_insert(Vec::new()).push(n);

        let n = monkeys.iter().find(|x| x.name == ed[2]).unwrap();
        neighbours.entry(&monkey).or_insert(Vec::new()).push(n);
    }
    neighbours
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Monkey {
    name: String,
    monkey_type: MonkeyType,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum MonkeyType {
    Number(u64),
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[derive(Debug)]
struct MonkeyNode {
    val: Option<u64>,
    children: Option<Vec<MonkeyNode>>,
    monkey_type: MonkeyType,
}