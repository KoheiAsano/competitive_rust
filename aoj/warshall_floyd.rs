// =========
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::process::exit;

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
// =========
fn warshall_floyd(adjm: &mut Vec<Vec<Option<i64>>>) {
    let n = adjm.len();
    // kを経て短くなる辺を全部短くする
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                // if it is connected,
                if let (Some(dik), Some(djk)) = (adjm[i][k], adjm[k][j]) {
                    if adjm[i][j].is_none() || adjm[i][j].unwrap() > dik + djk {
                        adjm[i][j] = Some(dik + djk);
                    }
                }
            }
        }
    }
}
fn main() {
    input! {
        v: usize,
        e:usize,
        std: [(usize,usize, i64);e],
    }
    let mut adjm = vec![vec![None; v]; v];
    for edge in std {
        adjm[edge.0][edge.1] = Some(edge.2);
    }
    for i in 0..v {
        adjm[i][i] = Some(0);
    }
    // println!("{:?}", adjm);
    warshall_floyd(&mut adjm);

    for i in 0..v {
        match adjm[i][i] {
            Some(d) => {
                if d < 0 {
                    println!("NEGATIVE CYCLE");
                    exit(0);
                }
            }
            None => (),
        }
    }
    for i in 0..v {
        for j in 0..v {
            match adjm[i][j] {
                Some(d) => print!("{}", d),
                None => print!("INF"),
            }
            if j != v - 1 {
                print!(" ");
            }
        }
        print!("\n");
    }
}

mod tests {
    use super::*;
    #[test]
    fn check() {}
}
