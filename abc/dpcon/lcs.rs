use std::cmp::max;
use std::collections::HashMap;
const MOD: usize = 1000000007;

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    // var... 変数の識別子, $t...型を一つよむ
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        //ここで繰り返し
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };
    //
    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };
    // 配列の最後のNestではここで型が指定されてparseされる
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input! {
        s: String,
        t: String,
    }
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    let n: usize = s.len();
    let m: usize = t.len();
    let mut memo: Vec<Vec<usize>> = vec![vec![0; m + 1]; n + 1];

    for i in 0..n {
        for j in 0..m {
            memo[i + 1][j + 1] = if s[i] == t[j] {
                max(max(memo[i][j] + 1, memo[i][j + 1]), memo[i + 1][j])
            } else {
                max(max(memo[i][j], memo[i][j + 1]), memo[i + 1][j])
            }
        }
    }
    let mut x = n;
    let mut y = m;
    let mut res: Vec<char> = vec![];
    while x > 0 && y > 0 {
        if memo[x][y] == memo[x - 1][y] {
            x -= 1;
        } else if memo[x][y] == memo[x][y - 1] {
            y -= 1;
        } else {
            x -= 1;
            y -= 1;
            res.push(s[x]);
        }
    }
    let mut ans = String::new();
    for c in res.iter().rev() {
        ans.push(*c);
    }
    println!("{}", ans);
}
