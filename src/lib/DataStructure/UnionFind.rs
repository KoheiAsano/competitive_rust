//
/*
サイズはVecにしちゃっている
*/

// ============
#[derive(Debug)]
struct UnionFind {
    // size= 親ならサイズ,その他は未定義. table=親を指す
    size: Vec<usize>,
    table: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        let size = vec![1; n];
        let mut table = vec![0; n];
        for i in 0..n {
            table[i] = i;
        }

        UnionFind { table: table, size }
    }
}
impl UnionFind {
    fn root(&mut self, x: usize) -> usize {
        // 負ならそれが親
        // 他のを指しているならたどる
        if self.table[x] == x {
            x
        } else {
            let tmp = self.table[x];
            self.table[x] = self.root(tmp);
            self.table[x]
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
        // ここは工夫していない思考停止でbにマージ
        self.table[a_root] = b_root;
        self.size[b_root] += self.size[a_root];
    }
    // 親のサイズを返す
    fn size(&mut self, x: usize) -> usize {
        let ri = self.root(x);
        self.size[ri]
    }
}

// ============

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn union_root_same() {
        let mut uf = UnionFind::new(10);
        // はじめは自分をさしてる
        for i in 0..10 {
            assert_eq!(uf.root(i), i);
        }
        // つなぐ
        uf.union(0, 1);
        uf.union(0, 0);
        uf.union(1, 2);
        uf.union(2, 9);

        assert_eq!(uf.same(0, 1), true);
        assert_eq!(uf.same(0, 2), true);
        assert_eq!(uf.same(0, 3), false);
        assert_eq!(uf.same(0, 4), false);
        assert_eq!(uf.same(0, 9), true);

        assert_eq!(uf.size(0), 4);
        assert_eq!(uf.size(3), 1);
    }

    #[test]
    fn check_size() {
        let mut uf = UnionFind::new(10);

        uf.union(1, 2);
        uf.union(4, 6);
        uf.union(9, 1);
        assert_eq!(uf.size(0), 1);
        assert_eq!(uf.size(1), 3);
        assert_eq!(uf.size(2), 3);
        assert_eq!(uf.size(3), 1);
        assert_eq!(uf.size(4), 2);
        assert_eq!(uf.size(5), 1);
        assert_eq!(uf.size(6), 2);
        assert_eq!(uf.size(9), 3);
    }

}

fn main() {}
