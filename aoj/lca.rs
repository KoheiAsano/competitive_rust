// =========
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::process::exit;

const MOD: usize = 1000000007;

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
// =========
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
// adjl...隣接リスト, u...今の点, depth...各頂点の深さを持つ, d...深さの値, fid...各頂点がはじめて出るet上のIndex
fn eulertour(
    adjl: &Vec<Vec<usize>>,
    u: usize,
    p: usize,
    et: &mut Vec<usize>,
    depth: &mut Vec<usize>,
    d: usize,
    fid: &mut Vec<usize>,
) {
    depth[u] = d;
    fid[u] = et.len();
    et.push(u);
    for v in &adjl[u] {
        if *v != p {
            eulertour(adjl, *v, u, et, depth, d + 1, fid);
            et.push(u);
        }
    }
}

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let mut adjl: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        let k: usize = sc.read();
        for _ in 0..k {
            let c: usize = sc.read();
            adjl[i].push(c);
        }
    }
    let mut et: Vec<usize> = vec![];
    let mut depth: Vec<usize> = vec![std::usize::MAX; n];
    let mut fid: Vec<usize> = vec![std::usize::MAX; n];
    eulertour(&adjl, 0, 0, &mut et, &mut depth, 0, &mut fid);
    let v = et
        .iter()
        .map(|e| (*e, depth[*e]))
        .collect::<Vec<(usize, usize)>>();

    // index, depthで持つ
    let mut st = SegTree::<(usize, usize)>::new(
        v,
        (std::usize::MAX, std::usize::MAX),
        Box::new(
            |l: (usize, usize), r: (usize, usize)| {
                if l.1 < r.1 {
                    l
                } else {
                    r
                }
            },
        ),
        Box::new(|_old: (usize, usize), new: (usize, usize)| new),
    );

    let q: usize = sc.read();

    // query 処理
    for _ in 0..q {
        let u: usize = sc.read();
        let v: usize = sc.read();
        if u == v {
            println!("{:?}", u);
        } else if fid[u] < fid[v] {
            println!("{:?}", st.query(fid[u], fid[v], 0, 0, st.num).0);
        } else {
            println!("{:?}", st.query(fid[v], fid[u], 0, 0, st.num).0);
        }
    }
}
