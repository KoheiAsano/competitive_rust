// =========
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::process::exit;

const MOD: i64 = 1000000007;

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
// =========

// mod mの体におけるaの逆元
macro_rules! impl_modinv {
    ($U:ty) => {
        fn mod_inv(a: $U, m: $U) -> $U {
            let mut ab = (a as i64, m as i64);
            let mut uv = (1, 0);
            let mut t: i64;
            while ab.1 != 0 {
                t = ab.0 / ab.1;
                ab = (ab.1, ab.0 - t * ab.1);
                uv = (uv.1, uv.0 - t * uv.1);
            }
            // if ab.0 != 1 {
            //     // panic!("{} and {} are not coprime g={}", a, m, ab.0);
            //     println!("{:?}", -1);
            //     exit(0);
            // }
            let inv = uv.0 % m as i64;
            if inv < 0 {
                (inv + m as i64) as $U
            } else {
                inv as $U
            }
        }
    };
}
fn gcd(a: i64, b: i64) -> i64 {
    let (a, b) = if a < b { (b, a) } else { (a, b) };
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

impl_modinv!(i64);
// mr[i].0 ... 互いに素
fn garner(mr: &mut Vec<(i64, i64)>, m: i64) -> i64 {
    mr.push((m, 0));
    // coef... mixed radixの係数, constants... 前まで求めた係数
    let mut coef: Vec<i64> = vec![1; mr.len()];
    let mut constants: Vec<i64> = vec![0; mr.len()];
    for i in 0..mr.len() - 1 {
        let mut v: i64 = (mr[i].1 - constants[i]) * mod_inv(coef[i], mr[i].0) % mr[i].0;
        if v < 0 {
            v += mr[i].0;
        }
        for j in i + 1..mr.len() {
            constants[j] += coef[j] * v;
            constants[j] %= mr[j].0;
            coef[j] *= mr[i].0;
            coef[j] %= mr[j].0;
        }
    }
    constants[mr.len() - 1]
}

fn main() {
    input! {
        n: usize,
        mr: [(i64,i64);n]
    }
    let mut lcm = false;
    for e in &mr {
        if e.0 == 0 {
            lcm = true;
        }
    }
    let mut mr = mr.iter().map(|e| (e.1, e.0)).collect::<Vec<(i64, i64)>>();
    // 前処理 m を互いに素にする
    for i in 0..mr.len() {
        for j in i + 1..mr.len() {
            let mut g = gcd(mr[i].0, mr[j].0);
            // 解の条件チェック
            if (mr[i].1 - mr[j].1) % g != 0 {
                println!("{:?}", -1);
                exit(0);
            }
            //ひとまず互いに素にする
            mr[i].0 /= g;
            mr[j].0 /= g;

            //gi...mr[j].0に残らないやつ(とりきってない)
            let mut gi = gcd(mr[i].0, g);
            //mr[j]に残るのは必ず含まれるけど不純
            let mut gj = g / gi;
            // giが取り切れなかったのをとっていく
            while g != 1 {
                g = gcd(gi, gj);
                gi *= g;
                gj /= g;
            }
            mr[i].0 *= gi;
            mr[j].0 *= gj;
            // あまりの更新
            mr[i].1 %= mr[i].0;
            mr[j].1 %= mr[j].0;
        }
    }
    let m: i64 = mr.iter().fold(1, |res, e| res * e.0 % MOD);
    let ans = garner(&mut mr, MOD);
    if lcm {
        println!("{:?}", m % MOD);
    } else {
        println!("{:?}", ans % MOD);
    }
}
