use std::collections::HashSet;

use reikna::{
    factor::{quick_factorize_wsp, MAX_SMALL_NUM},
    prime::prime_sieve,
};

pub fn d20() {
    d20_1();
    d20_2();
}

pub fn d20_1() {
    let n = 36000000;
    let mut h = 1;
    let primes = prime_sieve(MAX_SMALL_NUM);
    loop {
        let fs = quick_factorize_wsp(h, &primes);
        let a = HashSet::<u64>::from_iter(pm(fs));
        let c = (a.iter().sum::<u64>() + 1) * 10;
        if c >= n {
            break;
        }
        h += 1;
    }
    println!("{}", h);
}

pub fn d20_2() {
    let n = 36000000;
    let mut h = 1;
    let primes = prime_sieve(MAX_SMALL_NUM);
    loop {
        let fs = quick_factorize_wsp(h, &primes);
        let mut a = HashSet::<u64>::from_iter(pm(fs));
        a.insert(1);
        let c = (a.iter().filter(|&x| x * 50 >= h).sum::<u64>()) * 11;
        if c >= n {
            break;
        }
        h += 1;
    }
    println!("{}", h);
}

fn pm(l: Vec<u64>) -> Vec<u64> {
    let mut r = Vec::<u64>::new();
    if let Some(a) = l.iter().take(1).next() {
        for ele in pm(l.iter().skip(1).map(|x| *x).collect()) {
            r.push(ele);
            r.push(a * ele);
        }
        r.push(*a);
    }
    r
}