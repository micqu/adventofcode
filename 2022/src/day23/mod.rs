use crate::utils;

pub fn d23() {
    d23_1();
    // d23_2();
}

const PADDING: i32 = 200;
const DIR_X: [i32; 4] = [0, 0, -1, 1];
const DIR_Y: [i32; 4] = [-1, 1, 0, 0];
const ADJ: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
const ADJ_D: [[(i32, i32); 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
    [(-1, 1), (-1, 0), (-1, -1)],
    [(1, -1), (1, 0), (1, 1)],
];

pub fn d23_1() {
    let w = 7;
    let h = w;
    let mut map: Vec<Vec<u8>> = vec![vec![FREE; (PADDING * 2 + w) as usize]; (PADDING * 2 + h) as usize];
    let (mut xmin, mut ymin, mut xmax, mut ymax, n_elves) = parse("src/day23/input.txt", &mut map);

    let mut dir = 0;
    let mut prop: Vec<Vec<u8>> = vec![vec![0; (PADDING * 2 + w) as usize]; (PADDING * 2 + h) as usize];
    let mut to_move = Vec::<(Point, Point)>::with_capacity(n_elves as usize);

    for _ in 0..10 {
        // first half
        for x in xmin as usize..=xmax as usize {
            for y in ymin as usize..=ymax as usize {
                if map[y][x] == ELF {
                    if !ADJ.iter().any(|c| map[y + c.1 as usize][x + c.0 as usize] == ELF) {
                        continue;
                    }
                    
                    for i in 0..4 {
                        let d = (i + dir) % 4;
                        if ADJ_D[d].iter().all(|c| map[(y as i32 + c.1) as usize][(x as i32 + c.0) as usize] == FREE) {
                            prop[(y as i32 + DIR_Y[d]) as usize][(x as i32 + DIR_X[d]) as usize] += 1;
                            let from = Point { x: x as i32, y: y as i32};
                            let to = Point { x: (x as i32) + DIR_X[d], y: y as i32 + DIR_Y[d] };
                            to_move.push((from, to));
                            break;
                        }
                    }
                }
            }
        }

        // second half
        for (from, to) in to_move.iter() {
            if prop[to.y as usize][to.x as usize] == 1 {
                map[to.y as usize][to.x as usize] = ELF;
                map[from.y as usize][from.x as usize] = FREE;
                prop[to.y as usize][to.x as usize] = 0;

                xmin = xmin.min(to.x);
                xmax = xmax.max(to.x);
                ymin = ymin.min(to.y);
                ymax = ymax.max(to.y);
            }
        }
        to_move.clear();

        for a in &mut prop {
            for b in a {
                *b = 0;
            }
        }

        for j in ymin as usize..=ymax as usize {
            for i in xmin as usize..=xmax as usize {
                if map[j][i] == ELF {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("\n");

        dir += 1;
        dir %= 4;
    }

    println!("{}", (xmax - xmin + 1) * (ymax - ymin + 1) - n_elves);
}

pub fn d23_2() {
    let w = 7;
    let h = w;
    let mut map: Vec<Vec<u8>> = vec![vec![FREE; (PADDING * 2 + w) as usize]; (PADDING * 2 + h) as usize];
    let (mut xmin, mut ymin, mut xmax, mut ymax, n_elves) = parse("src/day23/input.txt", &mut map);

    let mut dir = 0;
    let mut prop: Vec<Vec<u8>> = vec![vec![0; (PADDING * 2 + w) as usize]; (PADDING * 2 + h) as usize];
    let mut to_move = Vec::<(Point, Point)>::with_capacity(n_elves as usize);

    let mut rounds = 0;
    loop {
        rounds += 1;
        // first half
        for x in xmin as usize..=xmax as usize {
            for y in ymin as usize..=ymax as usize {
                if map[y][x] == ELF {
                    if !ADJ.iter().any(|c| map[y + c.1 as usize][x + c.0 as usize] == ELF) {
                        continue;
                    }
                    
                    for i in 0..4 {
                        let d = (i + dir) % 4;
                        if ADJ_D[d].iter().all(|c| map[(y as i32 + c.1) as usize][(x as i32 + c.0) as usize] == FREE) {
                            prop[(y as i32 + DIR_Y[d]) as usize][(x as i32 + DIR_X[d]) as usize] += 1;
                            let from = Point { x: x as i32, y: y as i32};
                            let to = Point { x: (x as i32) + DIR_X[d], y: y as i32 + DIR_Y[d] };
                            to_move.push((from, to));
                            break;
                        }
                    }
                }
            }
        }

        // second half
        if to_move.is_empty() {
            break;
        }

        for (from, to) in to_move.iter() {
            if prop[to.y as usize][to.x as usize] == 1 {
                map[to.y as usize][to.x as usize] = ELF;
                map[from.y as usize][from.x as usize] = FREE;
                prop[to.y as usize][to.x as usize] = 0;

                xmin = xmin.min(to.x);
                xmax = xmax.max(to.x);
                ymin = ymin.min(to.y);
                ymax = ymax.max(to.y);
            }
        }
        to_move.clear();

        for a in &mut prop {
            for b in a {
                *b = 0;
            }
        }
        dir += 1;
        dir %= 4;
    }

    println!("{}", rounds);
}

const FREE: u8 = 0;
const ELF: u8 = 1;

fn parse(file: &str, elves: &mut Vec<Vec<u8>>) -> (i32, i32, i32, i32, i32) {
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    let (mut xmin, mut xmax) = (i32::MAX, 0);
    let (mut ymin, mut ymax) = (i32::MAX, 0);
    
    let mut n_elves = 0;
    let mut y = 0;
    for line in buffer.lines().filter(|x| !x.is_empty()) {
        let mut x = 0;
        for byte in line.chars() {
            match byte {
                '.' => { },
                '#' => {
                    let nx = x + PADDING;
                    let ny = y + PADDING;

                    xmin = xmin.min(nx);
                    xmax = xmax.max(nx);
                    ymin = ymin.min(ny);
                    ymax = ymax.max(ny);

                    elves[ny as usize][nx as usize] = ELF;
                    n_elves += 1;
                },
                _ => panic!(),
            }
            x += 1;
        }
        y += 1;
    }
    (xmin, ymin, xmax, ymax, n_elves)
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}