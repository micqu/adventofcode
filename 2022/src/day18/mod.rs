use crate::utils;

pub fn d18() {
    d18_1();
    d18_2();
}

pub fn d18_1() {
    let mut cubes: Vec<Cube> = Vec::new();
    parse("src/day18/input.txt", &mut cubes);

    let sides = calculate_surface_area(&cubes);
    println!("{}", sides);
}

pub fn d18_2() {
    let mut cubes: Vec<Cube> = Vec::new();
    parse("src/day18/input.txt", &mut cubes);
    
    let (mut xmin, mut xmax) = (i32::MAX, i32::MIN);
    let (mut ymin, mut ymax) = (i32::MAX, i32::MIN);
    let (mut zmin, mut zmax) = (i32::MAX, i32::MIN);

    cubes.iter().for_each(|c| {
        xmin = xmin.min(c.x);
        xmax = xmax.max(c.x);
        ymin = ymin.min(c.y);
        ymax = ymax.max(c.y);
        zmin = zmin.min(c.z);
        zmax = zmax.max(c.z);
    });

    let xd = (xmax - xmin) as usize + 2;
    let yd = (ymax - ymin) as usize + 2;
    let zd = (zmax - zmin) as usize + 2;
    
    let mut map: Vec::<Vec<Vec<u8>>> = vec![vec![vec![0; zd]; yd]; xd];

    cubes.iter().for_each(|c| {
        map[(c.x - xmin) as usize][(c.y - ymin) as usize][(c.z - zmin) as usize] = 1;
    });

    let sides = calculate_surface_area(&cubes);
    let mut flood_sides_all = Vec::<u32>::new();

    for i in 0..xd as usize {
        for j in 0..yd as usize {
            for k in 0..zd as usize {
                if map[i][j][k] >= 1 {
                    continue;
                }

                let start = Cube { x: i as i32, y: j as i32, z: k as i32 };
                let flood_cubes = flood3d(&start, 2, &mut map);
                let flood_sides = calculate_surface_area(&flood_cubes);
                flood_sides_all.push(flood_sides);
            }
        }
    }

    let result: u32 = flood_sides_all.iter().skip(1).sum();
    println!("{}", sides - result);
}

fn flood3d(start: &Cube, id: u8, map: &mut Vec::<Vec<Vec<u8>>>) -> Vec<Cube> {
    let mut cubes = Vec::<Cube>::new();
    let mut q = Vec::<Cube>::new();
    q.push(*start);

    while let Some(u) = q.pop() {
        if map[u.x as usize][u.y as usize][u.z as usize] == 0 {
            map[u.x as usize][u.y as usize][u.z as usize] = id;
            cubes.push(u);
        }
    
        let xmax = map.len() as i32;
        let ymax = map[0].len() as i32;
        let zmax = map[0][0].len() as i32;

        const ADJ: [(i32, i32, i32); 6] = [
            (1, 0, 0), (-1, 0, 0),
            (0, 1, 0), (0, -1, 0),
            (0, 0, 1), (0, 0, -1)
        ];

        ADJ.iter()
            .filter(|c|
                u.x + c.0 >= 0 && u.x + c.0 < xmax
                && u.y + c.1 >= 0 && u.y + c.1 < ymax
                && u.z + c.2 >= 0 && u.z + c.2 < zmax
                && map[(u.x + c.0) as usize][(u.y + c.1) as usize][(u.z + c.2) as usize] == 0 
            )
            .for_each(|c| {
                q.push(Cube {
                    x: u.x + c.0,
                    y: u.y + c.1,
                    z: u.z + c.2,
                });
            });    
    }

    cubes
}

fn calculate_surface_area(cubes: &Vec<Cube>) -> u32 {
    let mut sides = (cubes.len() * 6) as u32;
    
    for i in 0..cubes.len() - 1 {
        let a = &cubes[i];
        for j in i + 1..cubes.len() {
            let b = &cubes[j];
            if a.dist_l1(&b) == 1 {
                sides -= 2;
            }
        }
    }

    sides
}

fn parse<'a>(file: &str, cubes: &'a mut Vec<Cube>) {
    let mut buffer = String::new();
    utils::Reader::load_input(file).unwrap().read(&mut buffer);
    buffer.lines()
        .for_each(|x| {
            let k = x.split(',').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            cubes.push(Cube {
                x: k[0],
                y: k[1],
                z: k[2],
            });
        });
}

#[derive(Debug, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn dist_l1(&self, other: &Cube) -> i32 {
        (self.x - other.x).abs()
        + (self.y - other.y).abs()
        + (self.z - other.z).abs()
    }
}