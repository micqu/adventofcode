use std::collections::HashMap;

use crate::utils::{
    point2d::Point2d,
    solution::{IntoSolution, Solution},
};

pub const TITLE: &str = "Disk Fragmenter";
const INPUT: &'static str = include_str!("input.txt");

// pub fn part1() -> Option<Solution> {
//     let input = parse2();
//     let i = input.iter();
//     let j = input.iter().rev();



//     None
// }

pub fn part1() -> Option<Solution> {
    let files = parse();
    let mut output = Vec::<File>::with_capacity(files.len());
    let mut ri = files.len() - 1;
    let mut b = files[ri];
    let mut freed: usize = 0;
    for i in 0..files.len() {
        if i >= ri {
            break;
        }

        output.push(files[i]);

        if files[i].free == 0 {
            continue;
        }

        while i != ri && output.last().unwrap().free > 0 {
            if b.size == 0 {
                ri -= 1;
                if i != ri {
                    b = files[ri];
                }
                continue;
            }

            let a = output.last_mut().unwrap();
            let transfer_size = a.free.min(b.size);
            let rem = a.free - transfer_size;
            a.free = 0;

            output.push(File {
                id: b.id,
                size: transfer_size,
                free: rem,
                checked: true,
            });

            b.size -= transfer_size;
            freed += transfer_size as usize;
        }
    }

    if output.last().unwrap().free == 0 && b.size > 0 {
        let a = output.last_mut().unwrap();
        if a.id == b.id {
            a.size += b.size;
            a.free += b.free + freed;
        } else {
            b.free += freed;
            output.push(b);
        }
    }

    checksum(&output).solution()
}

pub fn part2() -> Option<Solution> {
    let input = parse2();
    let mut files = Vec::<(usize, usize, usize)>::new();
    let mut spaces = Vec::<(usize, usize, usize)>::new();
    let mut id = 0;
    let mut offset = 0;
    for (i, &v) in input.iter().enumerate() {
        if i % 2 == 0 {
            files.push((id, v, offset));
            id += 1;
        } else {
            spaces.push((i, v, offset));
        }
        offset += v;
    }

    let mut checksum = 0;
    let mut free: [usize; 10] = [0; 10];
    for &(i, size, offset) in files.iter().rev() {
        let mut placed = false;
        for j in free[size]..i {
            if spaces[j].1 >= size {
                free[size] = j;
                checksum += i * (spaces[j].2 * 2 + size - 1) * size / 2;
                spaces[j].1 -= size;
                spaces[j].2 += size;
                placed = true;
                break;
            }
        }
        if !placed {
            checksum += i * (offset * 2 + size - 1) * size / 2;
            free[size] = usize::MAX;
        }
    }

    checksum.solution()
}

fn checksum(files: &Vec<File>) -> usize {
    let mut checksum = 0;
    let mut pos = 0;
    for i in 0..files.len() {
        for j in 0..files[i].size {
            checksum += (pos + j) * files[i].id;
        }
        pos += files[i].allocated();
    }
    checksum
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: usize,
    size: usize,
    free: usize,
    checked: bool,
}

impl File {
    fn allocated(&self) -> usize {
        self.size + self.free
    }
}

fn parse2() -> Vec<usize> {
    INPUT.bytes().map(|x| (x - b'0') as usize).collect()
}

fn parse() -> Vec<File> {
    let mut files = Vec::<File>::new();
    let mut bytes = INPUT.bytes();
    let mut id = 0;
    while let Some(size) = bytes.next() {
        let free = bytes.next();
        files.push(File {
            id,
            size: (size as u8 - b'0') as usize,
            free: if let Some(f) = free {
                (f as u8 - b'0') as usize
            } else {
                0
            },
            checked: false,
        });
        id += 1;
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(), (6398252054886 as usize).solution());
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), (6415666220005 as usize).solution());
    }
}
