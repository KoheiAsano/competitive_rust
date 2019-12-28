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

fn main() {}

mod tests {
    use super::*;
    #[test]
    fn check_euler_tour() {
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
        // euler tour... DFSで辿る頂点を順に保存したもの
        let mut et: Vec<usize> = vec![];
        // 各頂点の深さ
        let mut depth: Vec<usize> = vec![std::usize::MAX; adjl.len()];
        // 各頂点が初めて現れるetのIndex
        let mut fid: Vec<usize> = vec![std::usize::MAX; adjl.len()];
        eulertour(&adjl, 0, 0, &mut et, &mut depth, 0, &mut fid);
        assert_eq!(et, vec![0, 1, 4, 1, 5, 6, 5, 7, 5, 1, 0, 2, 0, 3, 0]);
        println!("{:?}", et);
        assert_eq!(depth, vec![0, 1, 1, 1, 2, 2, 3, 3]);
        println!("{:?}", depth);
        assert_eq!(fid, vec![0, 1, 1, 1, 2, 2, 3, 3]);
        println!("{:?}", fid);
    }

    #[test]
    fn check_lca() {
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
        // euler tour
        let mut et: Vec<usize> = vec![];
        // 各頂点の深さ
        let mut depth: Vec<usize> = vec![std::usize::MAX; adjl.len()];
        // 各頂点が初めて現れるetのIndex
        let mut fid: Vec<usize> = vec![std::usize::MAX; adjl.len()];
        eulertour(&adjl, 0, 0, &mut et, &mut depth, 0, &mut fid);
        let v = et
            .iter()
            .map(|e| (*e, depth[*e]))
            .collect::<Vec<(usize, usize)>>();
        // オイラーツアーの最小値Indexを求めるセグ木
        let st = SegTree::<(usize, usize)>::new(
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
        let u = 6;
        let v = 7;
        if fid[u] < fid[v] {
            assert_eq!(5, st.query(fid[u], fid[v], 0, 0, st.num).0);
        } else {
            assert_eq!(5, st.query(fid[v], fid[u], 0, 0, st.num).0)
        }
    }

}
