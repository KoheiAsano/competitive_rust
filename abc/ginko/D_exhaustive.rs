fn main() {
    let n = read::<usize>();
    let s = read::<String>();
    let s: &[u8] = s.as_bytes();
    let mut ans = 0;
    let candidates: Vec<String> = (000)
        .map(|e| format!("{:0>3}", e.to_string()))
        .collect();
    for k in candidates {
        let k = k.as_bytes();
        let mut ok: bool = false;
        let mut cur: usize = 0;
        for l in 0..n {
            if s[l] == k[cur] {
                cur += 1;
            }
            if cur == 3 {
                ok = true;
                break;
            }
        }
        if ok {
            ans += 1
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
