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

fn directed_hierholder(adjl: &mut Vec<Vec<usize>>, start: usize) -> Option<Vec<usize>> {
    // res_circuitに確定したvertexを追加する
    let mut res_circuit: Vec<usize> = vec![];
    // これは一旦訪れたものを入れておくStack
    let mut trail_stack: Vec<usize> = vec![];
    trail_stack.push(start);

    let mut cur_v: usize = start;
    while trail_stack.len() > 0 {
        if adjl[cur_v].len() > 0 {
            trail_stack.push(cur_v);
            cur_v = adjl[cur_v].pop().unwrap()
        } else {
            res_circuit.push(cur_v);
            cur_v = trail_stack.pop().unwrap();
        }
    }
    // 有向なのでひっくりかえす
    res_circuit.reverse();
    Some(res_circuit)
}

fn main() {
    input! {
        n: usize,
        s:[String;n]
    };
    // dict
    let mut dic: HashMap<u64, String> = HashMap::new();
    for i in 0u8..=255 {
        for j in 0u8..=255 {
            let mut ss = String::new();
            ss.push(i as char);
            ss.push(j as char);
            dic.insert(i as u64 + j as u64 * 256, ss);
        }
    }

    // input c0c1c2 as edge(c0+c1*256,c1+c2*256)
    let mut adjl: Vec<Vec<usize>> = vec![vec![]; 256 * (256 + 1)];
    let mut outdeg: Vec<i64> = vec![0; 256 * (256 + 1)];
    let mut start = 0;
    for ss in s {
        let mut chars = ss.chars();
        let c1 = chars.next().unwrap() as usize;
        let c2 = chars.next().unwrap() as usize;
        let c3 = chars.next().unwrap() as usize;
        start = (c1 + c2 * 256) as usize;
        adjl[(c1 + c2 * 256) as usize].push(c2 + c3 * 256);
        outdeg[c1 + c2 * 256] += 1;
        outdeg[c2 + c3 * 256] -= 1;
    }

    // semi eulerian check
    // all 0 || two +1 and -1
    let mut onedeg: HashSet<i64> = HashSet::new();
    for (i, d) in outdeg.iter().enumerate() {
        if *d == 0 {
            continue;
        } else if *d == 1 && !onedeg.contains(&1) {
            start = i;
            onedeg.insert(1);
        } else if *d == -1 && !onedeg.contains(&-1) {
            onedeg.insert(-1);
        } else {
            println!("NO");
            exit(0);
        }
    }
    // if not connected, length != n+1
    let trail = directed_hierholder(&mut adjl, start).unwrap();
    if trail.len() != n + 1 {
        println!("NO");
        exit(0);
    }
    let mut res = dic[&(trail[0] as u64)].clone();
    for v in trail.iter().skip(1) {
        let mut chars = dic[&(*v as u64)].chars();
        //読み捨て
        chars.next();
        res = format!("{}{}", res, chars.next().unwrap())
    }
    println!("YES");
    println!("{}", res);
}
