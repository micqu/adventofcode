pub fn d10() {
    let mut ns: Vec<u32> = vec![1, 1, 1, 3, 2, 2, 2, 1, 1, 3];
    d10_1_2(&mut ns.clone(), 40);
    d10_1_2(&mut ns, 50);
}

pub fn d10_1_2(ns: &mut Vec<u32>, rounds: u32) {
    for _ in 0..rounds {
        let mut nns: Vec<u32> = Vec::new();
        let mut start = 0;
        while start < ns.len() {
            let mut nf = 1;
            let f = ns[start];
            for j in start + 1..ns.len() {
                if ns[j] == f {
                    nf += 1;
                } else {
                    break;
                }
            }
            start += nf as usize;
            nns.push(nf);
            nns.push(f);
        }
        
        ns.clear();
        ns.extend(nns);
    }
    println!("{}", ns.len());
}