#[derive(Debug, Clone)]
struct UnionFind {
    table: Vec<i32>,
    num: usize,
}
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            table: vec![-1; n],
            num: n,
        }
    }
    fn root(&mut self, x: usize) -> usize {
        let mut r = x;
        if self.table[x] >= 0 {
            // borrowing が厳しいコンパイラ向け
            let t = self.table[x] as usize;
            r = self.root(t);
            self.table[x] = r as i32;
        }
        r
    }
    fn find(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        (-self.table[r]) as usize
    }
    fn union(&mut self, x: usize, y: usize) {
        let xr = self.root(x);
        let yr = self.root(y);
        if xr != yr {
            if self.size(xr) < self.size(yr) {
                self.table[yr] += self.table[xr];
                self.table[xr] = yr as i32;
            } else {
                self.table[xr] += self.table[yr];
                self.table[yr] = xr as i32;
            }
            self.num -= 1;
        }
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
        println!("{:?}", uf);
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
