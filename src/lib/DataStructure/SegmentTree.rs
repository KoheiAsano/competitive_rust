// ============
struct SegTree<T> {
    // num: 葉の数, data: ノードの値, neutral: 単位元, operation: 区間クエリ, update: 点更新
    num: usize,
    data: Vec<T>,
    neutral: T,
    operation: Box<Fn(T, T) -> T>,
    update_point: Box<Fn(T, T) -> T>,
}

impl<T: Clone + Copy + std::fmt::Display> SegTree<T> {
    // v...元配列, neutral...初期値かつ単位元, operation...区間クエリ, update:
    fn new(
        v: Vec<T>,
        neutral: T,
        operation: Box<Fn(T, T) -> T>,
        update_point: Box<Fn(T, T) -> T>,
    ) -> Self {
        let n = v.len().checked_next_power_of_two().unwrap();
        println!("{:?}", n);
        // let n = 2 * i - 1;
        let mut data: Vec<T> = vec![neutral; 2 * n - 1];
        for i in 0..v.len() {
            data[i + n - 1] = v[i];
        }
        for i in (0..(n - 2)).rev() {
            data[i] = operation(data[2 * i + 1], data[2 * i + 2]);
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
        println!("{} {}", i, x);
        let mut i = i + self.num - 1; // 対応する葉のNodeへ
        println!("{:?}", i);
        self.data[i] = (self.update_point)(self.data[i], x);
        while i > 0 {
            println!("{:?}", i);
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
// ============

#[cfg(test)]
mod tests {
    use super::SegTree;

    #[test]
    fn check_min() {
        let mut st = SegTree::<usize>::new(
            vec![0, 0, 0],
            std::usize::MAX,
            Box::new(|l: usize, r: usize| -> usize { std::cmp::min(l, r) }),
            Box::new(|old: usize, new: usize| -> usize { new }),
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
        let mut st = SegTree::<usize>::new(
            vec![0, 0, 0],
            0,
            Box::new(|l: usize, r: usize| -> usize { l + r }),
            Box::new(|old: usize, new: usize| -> usize { new }),
        );
        st.update(0, 1);
        st.update(1, 2);
        st.update(2, 3);
        // [0,2)
        // [1,2)
        assert_eq!(st.query(0, 2, 0, 0, st.num), 3);
        assert_eq!(st.query(1, 2, 0, 0, st.num), 2);
    }

}
