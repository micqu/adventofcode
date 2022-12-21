use core::panic;

use crate::utils;

const ROCKS: [([u8; 4], u8, u8); 5] = [
        ([
            0b11110000,
            0b00000000,
            0b00000000,
            0b00000000
        ], 1, 4),
        ([
            0b01000000,
            0b11100000,
            0b01000000,
            0b00000000
        ], 3, 3),
        ([
            0b00100000,
            0b00100000,
            0b11100000,
            0b00000000
        ], 3, 3),
        ([
            0b10000000,
            0b10000000,
            0b10000000,
            0b10000000
        ], 4, 1),
        ([
            0b11000000,
            0b11000000,
            0b00000000,
            0b00000000
        ], 2, 2)
    ];

pub fn d17() {
    d17_1();
    d17_2();
}

pub fn d17_1() {
    let jets: Vec<Direction> = parse("src/day17/input.txt");
    let mut map: Vec<u8> = vec![0; 3300];
    let mut heights: Vec<u32> = vec![0; 7];
    
    let mut jet_idx: i32 = -1;
    for i in 0..2022 {
        let rock_type = ROCKS[i % ROCKS.len() as usize];
        let top = heights.iter().max().unwrap();

        let mut rock = Rock {
            x: 2,
            y: top + 3 + rock_type.1 as u32 - 1,
            shape: rock_type.0,
            h: rock_type.1,
            w: rock_type.2,
        };

        loop {
            jet_idx += 1;
            if jet_idx == jets.len() as i32 {
                jet_idx %= jets.len() as i32;
            }

            let dir = &jets[jet_idx as usize];
            if rock.is_place_free(dir, &map) {
                match dir {
                    Direction::Left => rock.x -= 1,
                    Direction::Right => rock.x += 1,
                    Direction::Down => {},
                };
            }

            if rock.is_place_free(&Direction::Down, &map) {
                rock.y -= 1;
            } else {
                break;
            }
        }

        rock.write_to(&mut map);

        let local_heights = rock.get_top();
        for i in 0..rock.w as usize {
            heights[i + rock.x as usize] = heights[i + rock.x as usize].max(local_heights[i] + 1);
        }
    }

    println!("{}", heights.iter().max().unwrap());
}

pub fn d17_2() {
    let jets: Vec<Direction> = parse("src/day17/input.txt");
    let mut map: Vec<u8> = vec![0; 10000];
    let mut cleared: u64 = 0;
    let mut heights: Vec<u32> = vec![0; 7];
    let mut jet_idx: i32 = -1;
    
    let chunk_len = 10;
    let x0 = 0;
    let mut tort = x0;
    let mut hare = x0 + 1;
    let mut cycle_count = 0;
    let mut rock_count_prev = 0;

    let mut remaining_rocks_after_skip = 0;
    let mut multi = 0;

    // const N_ROCKS: usize = 2022;
    const N_ROCKS: usize = 1000000000000;
    for i in 0..N_ROCKS {
        let rock_type = ROCKS[i % ROCKS.len() as usize];
        let top = *heights.iter().max().unwrap();

        let mut rock = Rock {
            x: 2,
            y: top + 3 + rock_type.1 as u32 - 1,
            shape: rock_type.0,
            h: rock_type.1,
            w: rock_type.2,
        };
        
        loop {
            jet_idx += 1;
            if jet_idx == jets.len() as i32 {
                jet_idx %= jets.len() as i32;
            }

            let dir = &jets[jet_idx as usize];
            if rock.is_place_free(dir, &map) {
                match dir {
                    Direction::Left => rock.x -= 1,
                    Direction::Right => rock.x += 1,
                    Direction::Down => {},
                };
            }
            
            if rock.is_place_free(&Direction::Down, &map) {
                rock.y -= 1;
            } else {
                break;
            }
        }

        rock.write_to(&mut map);
        
        let local_heights = rock.get_top();
        for i in 0..rock.w as usize {
            heights[i + rock.x as usize] = heights[i + rock.x as usize].max(local_heights[i] + 1);
        }

        let lowest = *heights.iter().min().unwrap();
        let all_low = heights.iter()
            .enumerate()
            .filter(|(_, &x)| x == lowest)
            .all(|(i, &x)| {
                let mut min = i;
                if i > 0 {
                    min = i - 1;
                }
                let mut max = i;
                if i < 6 {
                    max = i + 1;
                }
        
                let d1 = (x as i32 - heights[min as usize] as i32).abs();
                let d2 = (x as i32 - heights[max as usize] as i32).abs();
                d1 <= 1 && d2 <= 1
            });
        
        if lowest >= hare + chunk_len {
            tort += 1;
            hare += 2;

            let tort_d = &map[tort as usize..(tort + chunk_len) as usize];
            let hare_d = &map[hare as usize..(hare + chunk_len) as usize];

            if tort_d.iter().zip(hare_d.iter()).all(|(&a, &b)| a == b) {
                cycle_count += 1;
                
                if cycle_count == 1 {
                    rock_count_prev = i + 1;
                }
                
                if cycle_count == 2 {
                    let rock_count = i + 1;
                    let remaining_rocks = (N_ROCKS - rock_count) as u64;
                    let rocks_in_cycle = rock_count - rock_count_prev;
                    let rock_cycles = remaining_rocks / rocks_in_cycle as u64;
                    
                    let cycle_length = hare - tort;
                    multi = rock_cycles * cycle_length as u64;
                    let rocks_skipped = rocks_in_cycle as u64 * rock_cycles;
                    remaining_rocks_after_skip = remaining_rocks - rocks_skipped;
                    
                    break;
                }
            }
        }

        let slowest = lowest.min(tort);
        if all_low {
            map.drain(0..slowest as usize);
            cleared += slowest as u64;
            heights.iter_mut().for_each(|x| *x -= slowest);
            map.extend(vec![0; slowest as usize]);
            tort -= slowest;
            hare -= slowest;
        }
    }
    
    for i in 0..remaining_rocks_after_skip as usize {
        let rock_type = ROCKS[i % ROCKS.len() as usize];
        let top = *heights.iter().max().unwrap();

        let mut rock = Rock {
            x: 2,
            y: top + 3 + rock_type.1 as u32 - 1,
            shape: rock_type.0,
            h: rock_type.1,
            w: rock_type.2,
        };
        
        loop {
            jet_idx += 1;
            if jet_idx == jets.len() as i32 {
                jet_idx %= jets.len() as i32;
            }

            let dir = &jets[jet_idx as usize];
            if rock.is_place_free(dir, &map) {
                match dir {
                    Direction::Left => rock.x -= 1,
                    Direction::Right => rock.x += 1,
                    Direction::Down => {},
                };
            }
            
            if rock.is_place_free(&Direction::Down, &map) {
                rock.y -= 1;
            } else {
                break;
            }
        }

        rock.write_to(&mut map);
        
        let local_heights = rock.get_top();
        for i in 0..rock.w as usize {
            heights[i + rock.x as usize] = heights[i + rock.x as usize].max(local_heights[i] + 1);
        }

        let lowest = *heights.iter().min().unwrap();
        let all_low = heights.iter()
            .enumerate()
            .filter(|(_, &x)| x == lowest)
            .all(|(i, &x)| {
                let mut min = i;
                if i > 0 {
                    min = i - 1;
                }
                let mut max = i;
                if i < 6 {
                    max = i + 1;
                }
        
                let d1 = (x as i32 - heights[min as usize] as i32).abs();
                let d2 = (x as i32 - heights[max as usize] as i32).abs();
                d1 <= 1 && d2 <= 1
            });
        
        if all_low {
            map.drain(0..lowest as usize);
            cleared += lowest as u64;
            heights.iter_mut().for_each(|x| *x -= lowest);
            map.extend(vec![0; lowest as usize]);
        }
    }

    println!("{}", multi + cleared + *heights.iter().max().unwrap() as u64);
}

fn print_rock_map(rock: &mut Rock, map: &mut Vec<u8>, height: usize) {
    let mut k = map.clone();
    rock.print(&mut k);
    print(&k, height);
}

fn print(map: &Vec<u8>, height: usize) {
    for section_idx in (0..height).rev() {
        let section = map[section_idx];
        print!("|");
        for i in 0..7 {
            if section & (0b10000000 >> i) != 0 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
    println!("+-------+");
}


fn parse(file: &str) -> Vec<Direction> {
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    buffer.lines().next().unwrap().as_bytes().iter().map(|x| {
        if *x == '>' as u8 {
            return Direction::Right;
        }
        if *x == '<' as u8 {
            return Direction::Left;
        }
        panic!();
        
    }).collect()
}

#[derive(Debug, Clone)]
struct Rock {
    x: u32,
    y: u32,
    shape: [u8; 4],
    h: u8,
    w: u8,
}

impl Rock {
    fn is_place_free(&self, dir: &Direction, map: &Vec<u8>) -> bool {
        match dir {
            Direction::Left => {
                if self.x <= 0 {
                    return false;
                }

                let bbox_bottom = (self.y - self.h as u32 + 1) as usize;
                let bbox_top = self.y as usize;
                
                let map_section = &map[bbox_bottom..=bbox_top];

                for i in 0..map_section.len() {
                    let moved_shape = self.shape[i] >> (self.x - 1);
                    let hit_block = moved_shape & map_section[map_section.len() - i - 1] != 0;
                    if hit_block {
                        return false;
                    }
                }
                true
            },
            Direction::Right => {
                if self.x >= 7 - self.w as u32 {
                    return false;
                }

                let bbox_bottom = (self.y - self.h as u32 + 1) as usize;
                let bbox_top = self.y as usize;
                let map_section = &map[bbox_bottom..=bbox_top];
                
                for i in 0..map_section.len() {
                    let moved_shape = self.shape[i] >> (self.x + 1);
                    let hit_block = moved_shape & map_section[map_section.len() - i - 1] != 0;
                    if hit_block {
                        return false;
                    }
                }
                true
            },
            Direction::Down => {
                if self.y < self.h as u32 {
                    return false;
                }

                let bbox_bottom = (self.y - self.h as u32) as usize;
                let bbox_top = self.y as usize - 1;
                let map_section = &map[bbox_bottom..=bbox_top];
                
                for i in 0..map_section.len() {
                    let moved_shape = self.shape[i] >> self.x;
                    let hit_block = moved_shape & map_section[map_section.len() - i - 1] != 0;
                    if hit_block {
                        return false;
                    }
                }
                true
            },
        }
    }

    fn get_top(&self) -> [u32; 8] {
        let mut top: [u32; 8] = [0; 8];
        let mut checked: [bool; 8] = [false; 8];
        
        for i in 0..self.h as usize {
            for j in 0..self.w as usize {
                let cursor = 0b10000000 >> j;
                if cursor & self.shape[i] != 0 && !checked[j] {
                    top[j] = self.y - i as u32;
                    checked[j] = true;
                }
            }
        }
        top
    }

    fn write_to(&mut self, map: &mut Vec<u8>) {
        for i in 0..self.h as usize {
            map[(self.y - i as u32) as usize] |= self.shape[i] >> self.x;
        }
    }
    
    fn print(&mut self, map: &mut Vec<u8>) {
        for i in 0..self.h as usize {
            map[(self.y - i as u32) as usize] |= self.shape[i] >> self.x;
        }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down
}