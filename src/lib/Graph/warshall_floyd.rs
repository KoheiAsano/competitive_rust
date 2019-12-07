/*
matsuさんのwarshall-floyd
向き付き、経路を記録して最短経路を復元もできるように
*/

// distを更新する
// nextは中継地点が更新されたらその中継地点を記録する。たどるとPathが復元できる

fn warshall_floyd(dist: &mut Vec<Vec<Option<usize>>>) -> Vec<Vec<usize>> {
    let n = dist.len();
    let mut next = vec![];

    for _ in 0..n {
        next.push((0..n).map(|j| j).collect::<Vec<usize>>());
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(dik), Some(dkj)) = (dist[i][k], dist[k][j]) {
                    if dist[i][j].is_none() || dist[i][j].unwrap() > dik + dkj {
                        dist[i][j] = Some(dik + dkj);
                        next[i][j] = next[i][k];
                    }
                }
            }
        }
    }
    next
}

fn shortest_path(
    dist: &Vec<Vec<Option<usize>>>,
    next: &Vec<Vec<usize>>,
    start: usize,
    goal: usize,
) -> Option<(usize, Vec<usize>)> {
    if dist[start][goal].is_none() {
        return None;
    }

    let mut path = vec![start];
    let mut node = start;
    while node != goal {
        path.push(next[node][goal]);
        node = next[node][goal];
    }
    Some((dist[start][goal].unwrap(), path))
}

fn main() {
    // adj matrix
    let mut dist = vec![
        vec![None, Some(1), Some(10), None, None],
        vec![None, None, None, Some(2), None],
        vec![None, Some(1), None, Some(3), Some(1)],
        vec![Some(7), None, None, None, Some(2)],
        vec![None, None, None, None, None],
    ];
    // println!("{:?}", dist);
    let next = warshall_floyd(&mut dist);
    // println!("{:?}", next);
    assert_eq!(shortest_path(&dist, &next, 0, 1), Some((1, vec![0, 1])));
    assert_eq!(shortest_path(&dist, &next, 0, 3), Some((3, vec![0, 1, 3])));
    assert_eq!(shortest_path(&dist, &next, 3, 0), Some((7, vec![3, 0])));
    assert_eq!(
        shortest_path(&dist, &next, 0, 4),
        Some((5, vec![0, 1, 3, 4]))
    );
    assert_eq!(shortest_path(&dist, &next, 4, 0), None);
}
