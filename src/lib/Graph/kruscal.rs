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
            let tmp = self.root(self.table[x] as usize);
            self.table[x] = tmp as i64;
            tmp
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
struct Kruscal {
    min_spanning: Vec<Edge>,
}
impl Kruscal {
    // build minimum spanning tree
    fn build(edges: &mut Vec<Edge>) -> (Vec<Edge>, i64) {
        let mut uf = UnionFind::new(edges.len());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_kruscal() {
        let v = 6;
        let e = 9;
        // from, to, cost
        let data: Vec<(usize, usize, i64)> = vec![
            (0, 1, 1),
            (0, 2, 3),
            (1, 2, 1),
            (1, 3, 7),
            (2, 4, 1),
            (1, 4, 3),
            (3, 4, 1),
            (3, 5, 1),
            (4, 5, 6),
        ];
        let mut edges: Vec<Edge> = data
            .iter()
            .map(|(f, t, c): &(usize, usize, i64)| Edge {
                from: *f,
                to: *t,
                cost: *c,
            })
            .collect();
        let k = Kruscal::build(&mut edges);
        println!("{:?}", k);
    }

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
