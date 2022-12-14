use crate::utils;

pub fn d14() {
    let mut map = parse("src/day14/input.txt");
    // d14_1_sim(&mut map.clone());
    d14_1(&mut map.clone());
    d14_2(&mut map);
}

pub fn d14_1_sim(map: &mut Vec<Vec<u8>>) {
    let mut count = 0;
    let floor = 171;
    loop {
        let top = map.iter()
            .map(|x| x[500])
            .position(|x| x == 1)
            .unwrap() as i32;
        let mut sand: (i32, i32) = (500, top - 1);
        loop {
            let last = sand;
            let new_y = (sand.1 + 1) as usize;
            
            if new_y == floor {
                println!("{}", count);
                return;
            }
            
            if map[new_y][sand.0 as usize] == 0 {
                sand.1 += 1;
            } else if map[new_y][(sand.0 - 1) as usize] == 0 {
                sand.0 -= 1;
                sand.1 += 1;
            } else if map[new_y][(sand.0 + 1) as usize] == 0 {
                sand.0 += 1;
                sand.1 += 1;
            }
            
            if sand == last {
                break;
            }
        }
    
        map[sand.1 as usize][sand.0 as usize] = 1;
        count += 1;
        if sand.1 == 0 {
            break;
        }
    }
    println!("{}", count);
}

pub fn d14_1(map: &mut Vec<Vec<u8>>) {
    let mut c = 0;
    let mut q = Vec::<Node>::new();
    q.push(Node { x: 500, y: 0, cost: 0 });

    while let Some(v) = q.pop() {
        if v.y + 1 == 171 {
            println!("{}", c - v.cost);
            return;
        }
        
        c += 1;
        for n in v.get_neighbours_dfs(&map) {
            if map[n.y as usize][n.x as usize] == 0 {
                map[n.y as usize][n.x as usize] = 1;
                q.push(n);
            }
        }
    }
}

pub fn d14_2(map: &mut Vec<Vec<u8>>) {
    let mut c = 0;
    let mut q = Vec::<Node>::new();
    q.push(Node { x: 500, y: 0, cost: 0 });

    while let Some(v) = q.pop() {
        c += 1;
        for n in v.get_neighbours_bfs(&map) {
            if map[n.y as usize][n.x as usize] == 0 {
                map[n.y as usize][n.x as usize] = 1;
                q.push(n);
            }
        }
    }

    println!("{}", c);
}

fn parse(file: &str) -> Vec::<Vec<u8>> {
    let mut s = vec![vec![0; 700]; 171];
    let mut reader = utils::Reader::load_input(file).unwrap();
    let mut buffer = String::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        line.unwrap()
            .split("->")
            .map(|x| x
                .trim()
                .split(',')
                .collect::<Vec<&str>>()
            )
            .map(|x| (x[0].parse::<i32>().unwrap(), x[1].parse::<i32>().unwrap()))
            .collect::<Vec::<_>>()
            .windows(2)
            .for_each(|x| {
                let dx = x[1].0 - x[0].0;
                if dx > 0 {
                    for i in 0..=dx {
                        s[x[0].1 as usize][(x[0].0 + i) as usize] = 1;
                    }
                } else {
                    for i in (0..=-dx).rev() {
                        s[x[0].1 as usize][(x[0].0 - i) as usize] = 1;
                    }
                }
                let dy = x[1].1 - x[0].1;
                if dy > 0 {
                    for i in 0..=dy {
                        s[(x[0].1 + i) as usize][x[0].0 as usize] = 1;
                    }
                } else {
                    for i in (0..=-dy).rev() {
                        s[(x[0].1 - i) as usize][x[0].0 as usize] = 1;
                    }
                }
            });
    }
    s
}

struct Node {
    pub x: i16,
    pub y: i16,
    pub cost: u16,
}

impl Node {
    pub fn get_neighbours_bfs(&self, map: &Vec<Vec<u8>>) -> Vec<Node> {
        [(0, 1), (-1, 1), (1, 1)].iter()
            .map(|n| (self.x + n.0, self.y + n.1))
            .filter(|&n| n.1 < 171 && map[n.1 as usize][n.0 as usize] == 0)
            .map(|n| Node { x: n.0, y: n.1, cost: self.cost + 1 })
            .collect()
    }

    pub fn get_neighbours_dfs(&self, map: &Vec<Vec<u8>>) -> Vec<Node> {
        [(1, 1), (-1, 1), (0, 1)].iter()
            .map(|n| (self.x + n.0, self.y + n.1))
            .filter(|&n| n.1 < 171 && map[n.1 as usize][n.0 as usize] == 0)
            .map(|n| Node { x: n.0, y: n.1, cost: self.cost + 1 })
            .collect()
    }
}