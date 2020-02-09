fn main() {
    let n = read::<usize>();
    let s = read::<String>();
    let s: &[u8] = s.as_bytes();
    let mut ans = 0;
    // memo[pos][cur][string]
    let mut memo = [[[false; 1000]; 4]; 30001];
    for p in 0..n {
        for c in 0..4 {
            for ss in 000 {
                // その前の文字がない
                if memo[p][c][ss] == false {
                    continue;
                }
                // 選ばない
                memo[p + 1][c][ss] = true;
                // 選ぶ
                if c <= 2 {
                    memo[p + 1][c + 1][ss * 10 + ((s[p] as usize) - 48)] = true;
                }
            }
        }
    }
    for ss in 000 {
        if memo[n][3][ss] {
            ans += 1;
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
