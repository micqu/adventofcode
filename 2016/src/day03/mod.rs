use crate::{IntoSolution, Solution};

pub const TITLE: &str = "Squares With Three Sides";
const INPUT: &'static str = include_str!("input.txt");

pub fn part1() -> Option<Solution> {
    let triangles = parse();
    
    let mut count = 0;
    for mut triangle in triangles {
        triangle.sort_unstable_by(|a, b| a.length.cmp(&b.length));
        if triangle[0].length + triangle[1].length > triangle[2].length {
            count += 1;
        }
    }

    count.solution()
}

pub fn part2() -> Option<Solution> {
    let triangles = parse2();
    
    let mut count = 0;
    for mut triangle in triangles {
        triangle.sort_unstable_by(|a, b| a.length.cmp(&b.length));
        if triangle[0].length + triangle[1].length > triangle[2].length {
            count += 1;
        }
    }

    count.solution()
}

fn parse() -> Vec::<Triangle> {
    let mut triangles = Vec::<Triangle>::new();
    for line in INPUT.lines() {
        let sides = line.split_whitespace();

        let mut triangle = Triangle::new();

        for side in sides {
            let len = side.parse::<u32>().unwrap();
            triangle.push(Side { length: len });
        }

        triangles.push(triangle);
    }
    
    triangles
}

fn parse2() -> Vec::<Triangle> {
    let mut triangles = Vec::<Triangle>::new();
    let mut triple = vec![Triangle::new(), Triangle::new(), Triangle::new()];
    let mut count = 0;

    for line in INPUT.lines() {
        let sides = line.split_whitespace();

        for (i, side) in sides.enumerate() {
            let len = side.parse::<u32>().unwrap();
            triple[i].push(Side { length: len });
        }

        count += 1;

        if count >= 3 {
            triangles.append(&mut triple);
            triple = vec![Triangle::new(), Triangle::new(), Triangle::new()];
            count = 0;
        }
    }

    triangles
}

#[derive(Debug)]
struct Side {
    length: u32
}

type Triangle = Vec<Side>;