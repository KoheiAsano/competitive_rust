fn palindrome(s: String, n: usize, alp: &[u8]) {
    println!("{}", s);
    if s.len() == 0 {
        for letter in alp {
            palindrome(char::from(*letter).to_string(), n - 1, alp);
        }
    }
    if n > 1 {
        for letter in alp {
            palindrome(
                format!("{}{}{}", *letter as char, s, *letter as char).to_string(),
                n - 2,
                alp,
            );
        }
    }
}

fn main() {
    // matrix input
    // let nm: Vec<usize> = read_vec();
    // let mut map: Vec<Vec<usize>> = vec![vec![]; nm[0]];
    // for i in 0..nm[0] {
    //     map[i] = read_vec();
    // }
    // println!("{:?}", map);
    // palindrome("".to_string(), 8, "abcd".as_bytes());
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
