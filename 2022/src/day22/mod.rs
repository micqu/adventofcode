use std::collections::HashMap;

use crate::utils;

pub fn d22() {
    d22_1();
    d22_2();
}

pub fn d22_1() {
    let mut cmds: Vec<String> = Vec::new();
    let map = parse("src/day22/input.txt", &mut cmds);
    let h = map.len() as i32;
    let w = map[0].len() as i32;

    let mut dir: i32 = 0;
    let mut x: i32 = map[0].iter().position(|x| *x == FREE).unwrap() as i32;
    let mut y: i32 = 0;

    for cmd in cmds {
        let p = cmd.parse::<u32>();
        if p.is_ok() {
            for _ in 0..p.unwrap() {
                let mut nx = x + DIR_X[dir as usize];
                let mut ny = y + DIR_Y[dir as usize];

                if nx < 0 || nx >= w
                || ny < 0 || ny >= h
                || map[ny as usize][nx as usize] == EMPTY {
                    nx = x;
                    ny = y;
                    loop {
                        nx = ((nx + DIR_X[dir as usize]) % w + w) % w;
                        ny = ((ny + DIR_Y[dir as usize]) % h + h) % h;

                        if map[ny as usize][nx as usize] != EMPTY {
                            break;
                        }
                    }
                }

                if map[ny as usize][nx as usize] == FREE {
                    x = nx;
                    y = ny;
                    continue;
                }
                break;
            }
        } else {
            match cmd.chars().next().unwrap() {
                'L' => dir = add_dir(dir, -1),
                'R' => dir = add_dir(dir, 1),
                _ => panic!(),
            };
        }
    }
    println!("{}", 1000 * (y + 1) + 4 * (x + 1) + dir);
}

pub fn d22_2() {
    let mut cmds: Vec<String> = Vec::new();
    let map = parse("src/day22/input.txt", &mut cmds);
    let h = map.len() as i32;
    let w = map[0].len() as i32;

    const RIGHT: i32 = 0;
    const DOWN: i32 = 1;
    const LEFT: i32 = 2;
    const UP: i32 = 3;

    // Sample
    // let side_len = 4;
    // let sides: HashMap<u32, &Side> = [
    //     Side {
    //         idx: 6,
    //         up: (2, UP),
    //         down: (10, DOWN),
    //         left: (5, LEFT),
    //         right: (11, DOWN)
    //     },
    //     Side {
    //         idx: 4,
    //         up: (2, DOWN),
    //         down: (10, UP),
    //         left: (11, UP),
    //         right: (5, RIGHT)
    //     },
    //     Side {
    //         idx:5,
    //         up: (2, RIGHT),
    //         down: (10, RIGHT),
    //         left: (4, LEFT),
    //         right: (6, RIGHT)
    //     },
    //     Side {
    //         idx: 11,
    //         up: (6, LEFT),
    //         down: (4, RIGHT),
    //         left: (10, LEFT),
    //         right: (2, LEFT)
    //     },
    //     Side {
    //         idx: 2,
    //         up: (4, DOWN),
    //         down: (6, DOWN),
    //         left: (5, DOWN),
    //         right: (11, LEFT)
    //     },
    //     Side {
    //         idx: 10,
    //         up: (6, UP),
    //         down: (4, UP),
    //         left: (5, UP),
    //         right: (11, RIGHT)
    //     }
    // ].iter().map(|x| (x.idx, x)).collect::<HashMap<_, _>>();

    let side_len = 50;
    let sides: HashMap<u32, &Side> = [
        Side {
            idx: 7,
            up: (4, UP),
            down: (9, LEFT),
            left: (6, LEFT),
            right: (2, LEFT)
        },
        Side {
            idx: 1,
            up: (9, RIGHT),
            down: (4, DOWN),
            left: (6, RIGHT),
            right: (2, RIGHT)
        },
        Side {
            idx: 6,
            up: (4, RIGHT),
            down: (9, DOWN),
            left: (1, RIGHT),
            right: (7, RIGHT)
        },
        Side {
            idx: 2,
            up: (9, UP),
            down: (4, LEFT),
            left: (1, LEFT),
            right: (7, LEFT)
        },
        Side {
            idx: 4,
            up: (1, UP),
            down: (7, DOWN),
            left: (6, DOWN),
            right: (2, UP)
        },
        Side {
            idx: 9,
            up: (6, UP),
            down: (2, DOWN),
            left: (1, DOWN),
            right: (7, UP)
        }
    ].iter().map(|x| (x.idx, x)).collect::<HashMap<_, _>>();

    let mut dir: i32 = 0;
    let mut x: i32 = map[0].iter().position(|x| *x == FREE).unwrap() as i32;
    let mut y: i32 = 0;

    for cmd in cmds.iter() {
        let p = cmd.parse::<u32>();
        if p.is_ok() {
            for _ in 0..p.unwrap() {
                let current_side_idx = get_side_idx(x, y, side_len, w);
                let side = sides[&current_side_idx];

                let mut nx = x + DIR_X[dir as usize];
                let mut ny = y + DIR_Y[dir as usize];
                let mut next_side_idx = get_side_idx(nx, ny, side_len, w);

                let mut switching_side = false;
                if nx < 0 || nx >= w || ny < 0 || ny >= h || current_side_idx != next_side_idx {
                    next_side_idx = match dir {
                        RIGHT => side.right.0,
                        DOWN => side.down.0,
                        LEFT => side.left.0,
                        UP => side.up.0,
                        _ => panic!(),
                    };

                    let (from_corner_x, from_corner_y) = get_side_corner(current_side_idx, side_len, w);
                    let dx = x - from_corner_x;
                    let dy = y - from_corner_y;
                    
                    let (targ_corner_x, targ_corner_y) = get_side_corner(next_side_idx, side_len, w);

                    match dir {
                        RIGHT => {
                            if side.right.1 == LEFT {
                                nx = targ_corner_x + side_len - 1;
                                ny = targ_corner_y + (side_len - dy - 1);
                            }
                            if side.right.1 == RIGHT {
                                nx = targ_corner_x;
                                ny = targ_corner_y + dy;
                            }
                            if side.right.1 == UP {
                                nx = targ_corner_x + dy;
                                ny = targ_corner_y + side_len - 1;
                            }
                            if side.right.1 == DOWN {
                                nx = targ_corner_x + (side_len - dy - 1);
                                ny = targ_corner_y;

                            }
                        },
                        DOWN => {
                            if side.down.1 == UP {
                                nx = targ_corner_x + (side_len - dx - 1);
                                ny = targ_corner_y + side_len - 1;
                            }
                            if side.down.1 == DOWN {
                                nx = targ_corner_x + dx;
                                ny = targ_corner_y;
                            }
                            if side.down.1 == LEFT {
                                nx = targ_corner_x + side_len - 1;
                                ny = targ_corner_y + dx;
                            }
                            if side.down.1 == RIGHT {
                                nx = targ_corner_x;
                                ny = targ_corner_y + (side_len - dx - 1);
                            }
                        },
                        LEFT => {
                            if side.left.1 == LEFT {
                                nx = targ_corner_x + side_len - 1;
                                ny = targ_corner_y + dy;
                            }
                            if side.left.1 == RIGHT {
                                nx = targ_corner_x;
                                ny = targ_corner_y + (side_len - dy - 1);
                            }
                            if side.left.1 == UP {
                                nx = targ_corner_x + (side_len - dy - 1);
                                ny = targ_corner_y + side_len - 1;
                            }
                            if side.left.1 == DOWN {
                                nx = targ_corner_x + dy;
                                ny = targ_corner_y;
                            }
                        },
                        UP => {
                            if side.up.1 == UP {
                                nx = targ_corner_x + dx;
                                ny = targ_corner_y + side_len - 1;
                            }
                            if side.up.1 == DOWN {
                                nx = targ_corner_x + (side_len - dx - 1);
                                ny = targ_corner_y;
                            }
                            if side.up.1 == LEFT {
                                nx = targ_corner_x + side_len - 1;
                                ny = targ_corner_y + (side_len - dx - 1);
                            }
                            if side.up.1 == RIGHT {
                                nx = targ_corner_x;
                                ny = targ_corner_y + dx;
                            }
                        },
                        _ => panic!(),
                    }
                    switching_side = true;
                }
                
                if map[ny as usize][nx as usize] == FREE {
                    x = nx;
                    y = ny;
                    if switching_side {
                        match dir {
                            RIGHT => dir = side.right.1,
                            DOWN => dir = side.down.1,
                            LEFT => dir = side.left.1,
                            UP => dir = side.up.1,
                            _ => panic!(),
                        }
                    }
                    continue;
                }
                break;
            }
        } else {
            match cmd.chars().next().unwrap() {
                'L' => dir = add_dir(dir, -1),
                'R' => dir = add_dir(dir, 1),
                _ => panic!(),
            };
        }
    }
    println!("{}", 1000 * (y + 1) + 4 * (x + 1) + dir);
}

fn get_side_idx(x: i32, y: i32, s:i32, w: i32) -> u32 {
    let xd = x / s;
    let yd = y / s;
    (xd + (w / s) * yd) as u32
}

fn get_side_corner(idx: u32, s: i32, w: i32) -> (i32, i32) {
    (
        (idx as i32 % (w / s)) * s,
        (idx as i32 / (w / s)) * s
    )
}

fn add_dir(d: i32, nd: i32) -> i32 {
    ((d + nd) % 4 + 4) % 4
}

const EMPTY: u32 = 0;
const FREE: u32 = 1;
const WALL: u32 = 2;
const DIR_X: [i32; 4] = [1, 0, -1, 0];
const DIR_Y: [i32; 4] = [0, 1, 0, -1];

fn parse(file: &str, cmds: &mut Vec::<String>) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut w = 0;
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    for line in buffer.lines().filter(|x| !x.is_empty()) {
        let chars = line.chars().collect::<Vec<_>>();

        if !chars[0].is_ascii_digit() {
            let mut row = Vec::<u32>::new();
            for byte in line.chars() {
                row.push(match byte {
                    ' ' => EMPTY,
                    '.' => FREE,
                    '#' => WALL,
                    _ => panic!(),
                });
            }
            w = w.max(chars.len());
            map.push(row);
        } else {
            for cmd in split_keep(line) {
                cmds.push(cmd.to_owned());
            };
            
        }
    }
    for i in 0..map.len() {
        let d = w - map[i].len();
        if d > 0 {
            map[i].extend(vec![0 as u32; d]);
        }
    }
    map
}

fn split_keep<'a>(text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(|c: char| c.is_ascii_alphabetic()) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

#[derive(Debug)]
struct Side {
    idx: u32,
    up: (u32, i32),
    down: (u32, i32),
    left: (u32, i32),
    right: (u32, i32),
}