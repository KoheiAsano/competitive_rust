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
    pub fn vec<T: std::str::FromStr>(&mut self, n: u8) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
// =========

fn directed_hierholder(
    adjl: &mut HashMap<(u8, u8), Vec<(u8, u8)>>,
    start: (u8, u8),
) -> Option<Vec<(u8, u8)>> {
    let mut res_circuit: Vec<(u8, u8)> = vec![];
    let mut trail_stack: Vec<(u8, u8)> = vec![start];
    let mut cur_v: (u8, u8) = start;
    while trail_stack.len() > 0 {
        if adjl.get_mut(&cur_v).unwrap().len() > 0 {
            trail_stack.push(cur_v);
            match adjl.get_mut(&cur_v) {
                Some(n_list) => match n_list.pop() {
                    Some(n) => cur_v = n,
                    None => (),
                },
                None => (),
            }
        } else {
            res_circuit.push(cur_v);
            match trail_stack.pop() {
                Some(p) => cur_v = p,
                None => return None,
            }
        }
    }
    // 有向なのでひっくりかえす
    res_circuit.reverse();
    Some(res_circuit)
}

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();

    let mut deg = HashMap::<(u8, u8), isize>::new();
    // tuple のMapを初期化
    let mut adjl = {
        let mut adtmp = HashMap::<(u8, u8), Vec<(u8, u8)>>::new();
        for i in 0..255 {
            for j in 0..255 {
                adtmp.insert((i, j), vec![]);
            }
        }
        adtmp
    };
    // input を受け取る
    for _ in 0..n {
        let s: String = sc.read();
        let s = s.as_bytes();
        let start = (s[0], s[1]);
        let end = (s[s.len() - 2], s[s.len() - 1]);
        match adjl.get_mut(&start) {
            Some(l) => l.push(end),
            None => {
                adjl.insert(start, vec![end]);
            }
        }
        match deg.get_mut(&start) {
            Some(d) => *d += 1,
            None => {
                deg.insert(start, 1);
            }
        }
        match deg.get_mut(&end) {
            Some(d) => *d += -1,
            None => {
                deg.insert(end, -1);
            }
        }
    }
    // 始点を決める
    let start: (u8, u8) = {
        let mut s: (u8, u8) = (std::u8::MAX, std::u8::MAX);
        for (n, d) in &deg {
            if *d == 1 {
                if s != (std::u8::MAX, std::u8::MAX) {
                    println!("NO");
                    exit(0);
                }
                s = *n;
            } else if d.abs() > 1 {
                println!("NO");
                exit(0);
            }
        }
        if deg.get(&s) == None {
            *deg.iter().next().unwrap().0
        } else {
            s
        }
    };

    match directed_hierholder(&mut adjl, start) {
        Some(w) => {
            if w.len() != n + 1 {
                println!("NO");
                exit(0);
            }
            println!("YES");
            let mut pass = String::new();
            for s in w.iter() {
                pass.push(s.0 as char);
            }
            pass.push(w[w.len() - 1].1 as char);
            println!("{}", pass);
        }
        None => {
            println!("NO");
        }
    }
}
