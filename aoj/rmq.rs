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
struct SegTree<T> {
    // num: 葉の数, data: ノードの値, neutral: 単位元, operation: 区間クエリ, update: 点更新
    num: usize,
    data: Vec<T>,
    neutral: T,
    operation: Box<Fn(T, T) -> T>,
    update_point: Box<Fn(T, T) -> T>,
}

impl<T: Clone + Copy + std::fmt::Debug> SegTree<T> {
    // v...元配列, neutral...初期値かつ単位元, operation...区間クエリ, update:
    fn new(
        v: Vec<T>,
        neutral: T,
        operation: Box<Fn(T, T) -> T>,
        update_point: Box<Fn(T, T) -> T>,
    ) -> Self {
        let n = v.len().checked_next_power_of_two().unwrap();
        // let n = 2 * i - 1;
        let mut data: Vec<T> = vec![neutral; 2 * n - 1];
        for i in 0..v.len() {
            data[i + n - 1] = v[i];
        }
        if n > 1 {
            for i in (0..(n - 2)).rev() {
                data[i] = operation(data[2 * i + 1], data[2 * i + 2]);
            }
        }
        SegTree {
            num: n,
            data: data,
            neutral: neutral,
            operation: operation,
            update_point: update_point,
        }
    }
    // 点更新, i番目の値をxで更新
    fn update(&mut self, i: usize, x: T) {
        let mut i = i + self.num - 1; // 対応する葉のNodeへ
        self.data[i] = (self.update_point)(self.data[i], x);
        while i > 0 {
            i = (i - 1) / 2;
            // 親の値を更新する
            self.data[i] = (self.operation)(self.data[i * 2 + 1], self.data[i * 2 + 2]);
        }
    }
    // [a, b): クエリの区間, k: valueのNode, [l,r): k-Nodeの担当区間
    fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            self.neutral // 区間がかぶらないので単位元
        } else if a <= l && r <= b {
            self.data[k] //もろの区間なので値を返す
        } else {
            //半端な区間なので左右にqueryしてもう一回評価をする
            let l_val = self.query(a, b, 2 * k + 1, l, (l + r) / 2);
            let r_val = self.query(a, b, 2 * k + 2, (l + r) / 2, r);
            (self.operation)(l_val, r_val)
        }
    }
}
fn main() {
    input! {
        n: usize,
        q: usize,
        query: [(usize,usize,usize);q]
    }
    let mut st = SegTree::<usize>::new(
        vec![2usize.pow(31) - 1; n],
        2usize.pow(31) - 1,
        Box::new(|l: usize, r: usize| -> usize { std::cmp::min(l, r) }),
        Box::new(|old: usize, new: usize| -> usize { new }),
    );
    for q in query {
        if q.0 == 0 {
            st.update(q.1, q.2);
        } else {
            println!("{:?}", st.query(q.1, q.2 + 1, 0, 0, st.num));
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn check() {}

}
