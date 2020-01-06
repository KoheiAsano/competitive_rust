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
// use std::cmp::Ordering;
// ======UnionFind======
#[derive(Debug)]
struct UnionFind {
    // size= 親なら負のサイズ、子なら親
    // number= 集合の数
    table: Vec<i64>,
    number: usize,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        let mut table = vec![0; n];
        for i in 0..n {
            table[i] = -1;
        }
        UnionFind {
            table: table,
            number: n,
        }
    }
}
impl UnionFind {
    fn root(&mut self, x: usize) -> usize {
        // 負ならそれが親
        // 他のを指しているならたどる
        if self.table[x] < 0 {
            x
        } else {
            let tmp = self.table[x] as usize;
            self.table[x] = self.root(tmp) as i64;
            self.table[x] as usize
        }
    }
    fn same(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    fn union(&mut self, a: usize, b: usize) -> () {
        let a_root = self.root(a);
        let b_root = self.root(b);

        if a_root == b_root {
            return ();
        }
        // 負なので小さい法が大きい. 大きい方につける
        if self.table[a_root] > self.table[b_root] {
            self.table[b_root] += self.table[a_root];
            self.table[a_root] = b_root as i64;
        } else {
            self.table[a_root] += self.table[b_root];
            self.table[b_root] = a_root as i64;
        }
        self.number -= 1;
    }
    // 親のサイズを返す
    fn size(&mut self, x: usize) -> usize {
        let ri = self.root(x);
        -self.table[ri] as usize
    }
    fn count(&self) -> usize {
        self.number
    }
}

// ======Kruskal======
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug)]
struct Kruscal {}
impl Kruscal {
    // build minimum spanning tree
    fn build(v: usize, edges: &mut Vec<Edge>) -> (Vec<Edge>, i64) {
        let mut uf = UnionFind::new(v);
        // sort ascending order
        edges.sort();
        // remove duplicated edge
        edges.dedup();
        let mut res_tree: Vec<Edge> = vec![];
        let mut res: i64 = 0;
        // till graph is connected
        for e in edges {
            if !uf.same(e.from, e.to) {
                uf.union(e.from, e.to);
                res_tree.push(*e);
                res += e.cost;
            }
        }
        (res_tree, res)
    }
}

fn main() {
    input! {
        v: usize,
        e: usize,
        edata: [(usize,usize,i64);e]
    }
    // let mut edges: Vec<Edge> = edata
    //     .iter()
    //     .map(|(f, t, c)| Edge {
    //         from: *f,
    //         to: *t,
    //         cost: *c,
    //     })
    //     .collect();
    let mut edges = vec![];
    for (f, t, c) in edata {
        edges.push(Edge {
            from: f,
            to: t,
            cost: c,
        });
    }
    let k = Kruscal::build(v, &mut edges);
    println!("{:?}", k.1);
}
