use std::cmp::max;

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

fn accumulate_sum(v: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![0; v.len() + 1];
    for i in 1..res.len() {
        res[i] += v[i - 1] + res[i - 1];
    }
    res
}

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize);n]
    }
    let mut v0: Vec<usize> = vec![];
    let mut v1: Vec<usize> = vec![];
    let mut v2: Vec<usize> = vec![];
    let mut v3: Vec<usize> = vec![];
    for i in 0..n {
        match wv[i].0 - wv[0].0 {
            0 => v0.push(wv[i].1),
            1 => v1.push(wv[i].1),
            2 => v2.push(wv[i].1),
            3 => v3.push(wv[i].1),
            _ => (),
        }
    }
    v0.sort_by(|a, b| b.cmp(&a));
    let ac0 = accumulate_sum(&v0);
    v1.sort_by(|a, b| b.cmp(&a));
    let ac1 = accumulate_sum(&v1);
    v2.sort_by(|a, b| b.cmp(&a));
    let ac2 = accumulate_sum(&v2);
    v3.sort_by(|a, b| b.cmp(&a));
    let ac3 = accumulate_sum(&v3);
    let mut ans = 0;
    for i in 0..(v0.len() + 1) {
        for j in 0..(v1.len() + 1) {
            for k in 0..(v2.len() + 1) {
                for l in 0..(v3.len() + 1) {
                    let mut tmpans = 0;
                    tmpans += ac0[i] + ac1[j] + ac2[k] + ac3[l];
                    if w >= i * wv[0].0 + j * (wv[0].0 + 1) + k * (wv[0].0 + 2) + l * (wv[0].0 + 3)
                    {
                        ans = max(ans, tmpans);
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
