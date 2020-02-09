use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let s = read::<String>();
    let s = s.as_bytes();
    let mut k = Vec::<usize>::new();
    for i in 2..n {
        let mut kind = HashSet::<u8>::new();
        for ss in s.iter().skip(i) {
            kind.insert(*ss);
        }
        k.push(kind.len());
    }
    let mut comp = HashSet::<[u8; 2]>::new();
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            if comp.contains(&[s[i], s[j]]) {
                continue;
            } else {
                comp.insert([s[i], s[j]]);
                ans += if j != n - 1 { k[j - 1] } else { 0 };
            }
        }
    }
    println!("{}", ans);
}

#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
