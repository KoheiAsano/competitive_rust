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
use std::collections::BinaryHeap;

#[derive(Clone, Copy, PartialEq, Eq)]
struct NodeCand {
    cost: i64,
    vid: usize,
}
// to minimize binaryHeap, inverse order
impl Ord for NodeCand {
    fn cmp(&self, other: &NodeCand) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for NodeCand {
    fn partial_cmp(&self, other: &NodeCand) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Prim {}
impl Prim {
    // adjacency list is (cost, vid), 0-indexed, undirected
    fn build(adjl: Vec<Vec<(i64, usize)>>) -> i64 {
        // set managing spaninng tree
        let mut used: HashSet<usize> = HashSet::new();
        let mut heap: BinaryHeap<NodeCand> = BinaryHeap::new();
        let mut total = 0;

        // 0 as start
        heap.push(NodeCand { cost: 0, vid: 0 });

        while let Some(NodeCand { cost, vid }) = heap.pop() {
            if used.contains(&vid) {
                continue;
            } else {
                used.insert(vid);
                total += cost;
                for n in &adjl[vid] {
                    if !used.contains(&n.1) {
                        heap.push(NodeCand {
                            cost: n.0,
                            vid: n.1,
                        });
                    }
                }
            }
        }
        total
    }
}

fn main() {
    input! {
        n:usize,
        adjm: [[i64;n];n]
    }
    let mut adjl = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if adjm[i][j] != -1 {
                adjl[i].push((adjm[i][j], j));
            }
        }
    }
    println!("{:?}", Prim::build(adjl));
}
