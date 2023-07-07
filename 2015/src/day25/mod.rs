pub fn d25() {
    d25_1();
}

pub fn d25_1() {
    let a = 20151125;
    let r = 2978;
    let c = 3083;

    let idx = get_idx(c, r) - 1;
    
    let mut n = a;
    for _ in 0..idx {
        n = f(n);
    }
    println!("{}", n);
}

fn get_idx(x: u32, y: u32) -> u32 {
    let tw = x + y - 2;
    let a = tw * (tw + 1) / 2;
    x + a
}

fn f(n: u64) -> u64 {
    (n * 252533) % 33554393
}