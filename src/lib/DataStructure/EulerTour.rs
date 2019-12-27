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

fn eulertour(adjl: &Vec<Vec<usize>>, u: usize, et: &mut Vec<usize>) {
    et.push(u);
    for v in &adjl[u] {
        eulertour(adjl, *v, et);
        et.push(u);
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
    println!("{:?}", adjl);
    let mut et: Vec<usize> = vec![];
    eulertour(&adjl, 0, &mut et);
    println!("{:?}", et);

    let q: usize = sc.read();

    // query 処理
    for i in 0..q {
        let u: usize = sc.read();
        let v: usize = sc.read();
    }
}

mod tests {
    use super::*;
    #[test]
    fn check() {
        let adjl = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![],
            vec![],
            vec![],
            vec![6, 7],
            vec![],
            vec![],
        ];
        let mut et: Vec<usize> = vec![];
        eulertour(&adjl, 0, &mut et);
        assert_eq!(et, vec![0, 1, 4, 1, 5, 6, 5, 7, 5, 1, 0, 2, 0, 3, 0])
    }

}
