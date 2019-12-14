// ============

struct SegTree {
    // size: 葉の数, velue: ノードの値, neutral: 単位元, calc: 計算関数(isize, isize) -> isize
    num: usize,
    value: Vec<isize>,
    neutral: isize,
    calc: Box<Fn((isize, isize)) -> isize>,
}

impl SegTree {
    fn new(size: usize, neutral: isize, calc: Box<Fn((isize, isize)) -> isize>) -> Self {
        // セグ木にできる分のNodeの数を計算
        // size以上の最小の(2の冪の2倍-1)にしたい
        let i = size.checked_next_power_of_two().unwrap();
        let n = 2 * i - 1;
        let value: Vec<isize> = (0..n).map(|_| neutral).collect();
        SegTree {
            num: i,
            value: value,
            neutral: neutral,
            calc: calc,
        }
    }
    // 点更新, i番目の値をxで更新
    fn update(&mut self, i: usize, x: isize) {
        let mut i = i + self.num - 1; // 対応する葉のNodeへ
        self.value[i] = x;
        while i > 0 {
            i = (i - 1) / 2;
            // 親の値を更新する
            self.value[i] = (self.calc)((self.value[i * 2 + 1], self.value[i * 2 + 2]));
        }
    }
    // [a, b): クエリの区間, k: valueのNode, [l,r): k-Nodeの担当区間
    fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> isize {
        if r <= a || b <= l {
            self.neutral // 区間がかぶらないので単位元
        } else if a <= l && r <= b {
            self.value[k] //もろの区間なので値を返す
        } else {
            //半端な区間なので左右にqueryしてもう一回評価をする
            let l_val = self.query(a, b, 2 * k + 1, l, (l + r) / 2);
            let r_val = self.query(a, b, 2 * k + 2, (l + r) / 2, r);
            (self.calc)((l_val, r_val))
        }
    }
}
// ============
fn main() {
}

#[cfg(test)]
mod tests {
    use super::SegTree;

    #[test]
    fn check_min() {
        let mut st = SegTree::new(
            3,
            std::isize::MAX,
            Box::new(|(a, b): (isize, isize)| -> isize { std::cmp::min(a, b) }),
        );
        st.update(0, 1);
        st.update(1, 2);
        st.update(2, 3);
        // 1-indexedで来たらa-1, b
        // 0-indexedで来たらa, b+1
        assert_eq!(st.query(0, 3, 0, 0, st.num), 1);
        assert_eq!(st.query(1, 3, 0, 0, st.num), 2);
    }

    #[test]
    fn check_sum() {
        let mut st = SegTree::new(3, 0, Box::new(|(a, b): (isize, isize)| -> isize { a + b }));
        st.update(0, 1);
        st.update(1, 2);
        st.update(2, 3);
        // [0,2)
        // [1,2)
        assert_eq!(st.query(0, 2, 0, 0, st.num), 3);
        assert_eq!(st.query(1, 2, 0, 0, st.num), 2);
    }

}
