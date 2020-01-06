use std::collections::BinaryHeap;
use std::collections::HashSet;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_prim() {
        let n = 5;
        let adjl: Vec<Vec<(i64, usize)>> = vec![
            vec![(2, 1), (3, 2), (1, 3)],
            vec![(2, 0), (4, 3)],
            vec![(3, 0), (1, 3), (1, 4)],
            vec![(1, 0), (4, 1), (1, 2), (3, 4)],
            vec![(1, 2), (3, 3)],
        ];
        let p = Prim::build(adjl);
        println!("{:?}", p);
    }
}

fn main() {}
