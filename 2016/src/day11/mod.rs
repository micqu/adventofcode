use crate::Solution;

// type Floor = HashSet::<Component>;
// type Elevator = HashSet::<Component>;

pub const TITLE: &str = "Radioisotope Thermoelectric Generators";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    // let mut elevator = Elevator::new();
    // let mut pos = 0;

    // let mut floors = vec![ Floor::new(), Floor::new(), Floor::new(), Floor::new() ];
    // // 0: promethium, 1: cobalt, 2: curium, 3: ruthenium, 4: plutonium
    // floors[0].insert(Component { id: 0, component_type: ComponentType::Generator });
    // floors[0].insert(Component { id: 0, component_type: ComponentType::Microchip });
    // floors[1].insert(Component { id: 1, component_type: ComponentType::Generator });
    // floors[1].insert(Component { id: 2, component_type: ComponentType::Generator });
    // floors[1].insert(Component { id: 3, component_type: ComponentType::Generator });
    // floors[1].insert(Component { id: 4, component_type: ComponentType::Generator });
    // floors[2].insert(Component { id: 1, component_type: ComponentType::Microchip });
    // floors[2].insert(Component { id: 2, component_type: ComponentType::Microchip });
    // floors[2].insert(Component { id: 3, component_type: ComponentType::Microchip });
    // floors[2].insert(Component { id: 4, component_type: ComponentType::Microchip });

    // let s = solve(elevator, pos, floors);
    // dbg!(s);
    None
}

pub fn part2() -> Option<Solution> {
    None
}

// fn solve(elevator: Elevator, pos: u32, floors: Vec<Floor>) -> Option<u32> {
//     let q = Vec::<
//     if floors[3].len() == 10 {
//         break;
//     }

//     None
// }

// pub fn d10_2() {
// }

// #[derive(Debug, Eq, PartialEq)]
// struct State {
//     elevator: Elevator,
//     pos: u32,
//     floors: Vec<Floor>,
// }

// #[derive(Debug, Eq, PartialEq, Hash)]
// struct Component {
//     id: u32,
//     component_type: ComponentType,
// }

// #[derive(Debug, Eq, PartialEq, Hash)]
// enum ComponentType {
//     Generator,
//     Microchip,
// }