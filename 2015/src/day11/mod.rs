use std::collections::HashSet;

pub fn d11() {
    d11_1("hxbxwxba");
    d11_1("hxbxxyzz");
}

pub fn d11_1(pass: &str) {
    let mut a: Vec<u8> = pass.as_bytes().iter().rev().map(|x| *x - 97).collect();
    let invalid_chars = HashSet::<u8>::from_iter(vec!['i' as u8 - 97, 'o' as u8 - 97, 'l' as u8 - 97]);
    loop {
        incr(&mut a);
        let r1 = a.windows(3).any(|x| x[0] == x[1] + 1 && x[1] == x[2] + 1);
        let r2 = !a.iter().any(|x| invalid_chars.contains(x));
        let r3 = a.windows(2)
            .filter(|&x| x.len() == 2 && x[0] == x[1])
            .collect::<HashSet<_>>()
            .iter().count() >= 2;

        if r1 && r2 && r3 {
            break;
        }
    }
    let r = String::from_iter(a.iter().rev().map(|x| (x + 97) as char));
    println!("{}", r);
}

fn incr(a: &mut Vec<u8>) {
    a[0] += 1;
    if a.len() == 1 {
        if a[0] >= 26 {
            a.push(1);
            a[0] = 0;
        }
    } else {
        for i in 0..a.len() {
            if a[i] >= 26 {
                a[i] = 0;
                if i + 1 >= a.len() {
                    a.push(0);
                }
                a[i + 1] += 1;
            }
        }
    }
}