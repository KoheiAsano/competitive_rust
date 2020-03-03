
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adjl: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adjl.len()).map(|_| std::usize::MAX).collect();

    let mut heap = std::collections::BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for edge in &adjl[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
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
    let (v, e, start): (usize, usize, usize) = (read(), read(), read());
    let mut adjl = vec![vec![]; v];
    for _ in 0..e {
        let (from, to, cost): (usize, usize, usize) = (read(), read(), read());
        adjl[from].push(Edge {
            node: to,
            cost: cost,
        });
    }
    for i in 0..v {
        match shortest_path(&adjl, start, i) {
            Some(d) => println!("{}", d),
            None => println!("INF"),
        }
    }