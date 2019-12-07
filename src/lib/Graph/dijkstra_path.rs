use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
    pre_node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(
    adj_ist: &Vec<Vec<Edge>>,
    start: usize,
    goal: usize,
) -> Option<(usize, Vec<usize>)> {
    let mut dist: Vec<_> = (0..adj_ist.len()).map(|_| usize::MAX).collect();
    let mut pre_nodes: Vec<_> = (0..adj_ist.len()).map(|i| i).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
        pre_node: 0,
    });

    while let Some(State {
        cost,
        position,
        pre_node,
    }) = heap.pop()
    {
        if cost > dist[position] {
            continue;
        }
        pre_nodes[position] = pre_node;
        if position == goal {
            let mut v = goal;
            let mut path = vec![goal];
            // はじめまでたどる
            while v != start {
                path.push(pre_nodes[v]);
                v = pre_nodes[v];
            }
            path.reverse();
            return Some((cost, path));
        }

        for edge in &adj_ist[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
                pre_node: position,
            };
            // 次に行けるところがdistに書いたのより早く行ける
            if next.cost < dist[next.position] {
                heap.push(next);

                dist[next.position] = next.cost;
            }
        }
    }
    None
}

fn main() {
    let graph = vec![
        vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
        // Node 1
        vec![Edge { node: 3, cost: 2 }],
        // Node 2
        vec![
            Edge { node: 1, cost: 1 },
            Edge { node: 3, cost: 3 },
            Edge { node: 4, cost: 1 },
        ],
        // Node 3
        vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
        // Node 4
        vec![],
    ];

    assert_eq!(shortest_path(&graph, 0, 1), Some((1, vec![0, 1])));
    assert_eq!(shortest_path(&graph, 0, 3), Some((3, vec![0, 1, 3])));
    assert_eq!(shortest_path(&graph, 3, 0), Some((7, vec![3, 0])));
    assert_eq!(shortest_path(&graph, 0, 4), Some((5, vec![0, 1, 3, 4])));
    assert_eq!(shortest_path(&graph, 4, 0), None);
}
