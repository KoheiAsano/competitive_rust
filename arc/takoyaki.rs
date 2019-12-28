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
// ============
// ============
struct SegTree<T> {
    // num: 葉(元データ)の数, data: ノードの値, neutral: 単位元, merge: 区間クエリ, update_point: 点更新
    num: usize,
    data: Vec<T>,
    neutral: T,
    merge: Box<Fn(T, T) -> T>,
    update_point: Box<Fn(T, T) -> T>,
}

impl<T: Clone + Copy + std::fmt::Debug> SegTree<T> {
    // v...元配列, neutral...初期値かつ単位元, merge...区間クエリ, update:
    fn new(
        v: Vec<T>,
        neutral: T,
        merge: Box<Fn(T, T) -> T>,
        update_point: Box<Fn(T, T) -> T>,
    ) -> Self {
        let n = v.len().checked_next_power_of_two().unwrap();
        let mut data: Vec<T> = vec![neutral; 2 * n - 1];
        for i in 0..v.len() {
            data[i + n - 1] = v[i];
        }
        if n > 1 {
            for i in (0..(n - 2)).rev() {
                data[i] = merge(data[2 * i + 1], data[2 * i + 2]);
            }
        }
        SegTree {
            num: n,
            data: data,
            neutral: neutral,
            merge: merge,
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
            self.data[i] = (self.merge)(self.data[i * 2 + 1], self.data[i * 2 + 2]);
        }
    }
    // [a, b): クエリの区間, k: valueのNode, [l,r): k-Nodeの担当区間
    // 0-indexedで来たら[a, b+1]をする
    fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            self.neutral // 区間がかぶらないので単位元
        } else if a <= l && r <= b {
            self.data[k] //もろの区間なので値を返す
        } else {
            //半端な区間なので左右にqueryしてもう一回評価をする
            let l_val = self.query(a, b, 2 * k + 1, l, (l + r) / 2);
            let r_val = self.query(a, b, 2 * k + 2, (l + r) / 2, r);
            (self.merge)(l_val, r_val)
        }
    }
}
// ============

fn main() {
    input! {
        _n: usize,
        m: usize,
        query: [(usize, f64, f64);m]
    }
    // 座圧
    let mut ids = query.iter().map(|q| q.0).collect::<Vec<usize>>();
    ids.sort();
    ids.dedup();
    let ids = ids
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect::<HashMap<usize, usize>>();

    let mut st = SegTree::<(f64, f64)>::new(
        vec![(1.0, 0.0); ids.len()],
        (1.0, 0.0),
        Box::new(|l, r| (l.0 * r.0, l.1 * r.0 + r.1)),
        Box::new(|_old, new| new),
    );
    let mut minans = 1.0f64;
    let mut maxans = 1.0f64;
    let mut tmp: (f64, f64);
    for q in query {
        st.update(ids[&q.0], (q.1, q.2));
        tmp = st.query(0, st.num, 0, 0, st.num);
        minans = minans.min(tmp.0 * 1.0 + tmp.1);
        maxans = maxans.max(tmp.0 * 1.0 + tmp.1);
    }
    println!("{:?}", minans);
    println!("{:?}", maxans);
}
