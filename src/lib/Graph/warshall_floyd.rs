//======
// all-pair shortest path
// if u-u (self loop) is nothing, then Some(0)
// represent non-adjacent by None
// if result u-u(self loop) contain negative, then infinitely become short
fn warshall_floyd(adjm: &mut Vec<Vec<Option<i64>>>) {
    let n = adjm.len();
    // kを経て短くなる辺を全部短くする
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(dik), Some(djk)) = (adjm[i][k], adjm[k][j]) {
                    if adjm[i][j].is_none() || adjm[i][j].unwrap() > dik + djk {
                        adjm[i][j] = Some(dik + djk);
                    }
                }
            }
        }
    }
}

fn warshall_floyd_next(adjm: &mut Vec<Vec<Option<i64>>>) -> Vec<Vec<Option<usize>>> {
    let n = adjm.len();
    // Optionの行列、行けないならNone, 行けるなら次の点のIndex
    let mut next: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];
    //今ある辺で初期化
    for i in 0..n {
        for j in 0..n {
            match adjm[i][j] {
                Some(_) => next[i][j] = Some(j),
                None => (),
            }
        }
    }

    // kを経て短くなる辺を全部短くする
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(dik), Some(djk)) = (adjm[i][k], adjm[k][j]) {
                    if adjm[i][j].is_none() || adjm[i][j].unwrap() > dik + djk {
                        adjm[i][j] = Some(dik + djk);
                        // 一つの辺でいけるわけでないので、kではだめ
                        // adjm[i][k]がNoneでないならnext[i][k]はNoneでない
                        next[i][j] = next[i][k];
                    }
                }
            }
        }
    }
    next
}
//始点を除いた最短PATHを返す
fn warshall_floyd_path(i: usize, j: usize, next: &Vec<Vec<Option<usize>>>) -> Option<Vec<usize>> {
    let mut cur = i;
    let mut res = vec![];
    while cur != j {
        match next[cur][j] {
            Some(id) => {
                cur = id;
                res.push(id);
            }
            None => return None,
        }
    }
    Some(res)
}

#[test]
fn check_positive_undirected() {
    let v = 3;
    let mut adjm: Vec<Vec<Option<i64>>> = vec![
        vec![None, Some(1), Some(3)],
        vec![Some(1), None, Some(1)],
        vec![Some(3), Some(1), None],
    ];
    // should be zero not self loop
    for i in 0..v {
        match adjm[i][i] {
            Some(_d) => (),
            None => adjm[i][i] = Some(0),
        }
    }
    warshall_floyd(&mut adjm);
    println!("{:?}", adjm);
}
#[test]
fn check_positive_directed() {
    let v = 4;
    let mut adjm: Vec<Vec<Option<i64>>> = vec![
        vec![None, Some(1), Some(5), None],
        vec![None, None, Some(2), Some(4)],
        vec![None, None, None, Some(1)],
        vec![None, None, Some(7), None],
    ];
    // should be zero not self loop
    for i in 0..v {
        match adjm[i][i] {
            Some(d) => (),
            None => adjm[i][i] = Some(0),
        }
    }
    warshall_floyd(&mut adjm);

    assert_eq!(
        vec![
            vec![Some(0), Some(1), Some(3), Some(4)],
            vec![None, Some(0), Some(2), Some(3)],
            vec![None, None, Some(0), Some(1)],
            vec![None, None, Some(7), Some(0)]
        ],
        adjm
    );
    for i in 0..v {
        for j in 0..v {
            match adjm[i][j] {
                Some(d) => print!("{}", d),
                None => print!("INF"),
            }
            if j != v - 1 {
                print!(" ");
            }
        }
        print!("\n");
    }
}

#[test]
fn check_negative_directed() {
    let v = 4;
    let mut adjm: Vec<Vec<Option<i64>>> = vec![
        vec![None, Some(1), Some(-5), None],
        vec![None, None, Some(2), Some(4)],
        vec![None, None, None, Some(1)],
        vec![None, None, Some(7), None],
    ];
    // should be zero not self loop
    for i in 0..v {
        match adjm[i][i] {
            Some(_d) => (),
            None => adjm[i][i] = Some(0),
        }
    }
    warshall_floyd(&mut adjm);

    assert_eq!(
        vec![
            vec![Some(0), Some(1), Some(-5), Some(-4)],
            vec![None, Some(0), Some(2), Some(3)],
            vec![None, None, Some(0), Some(1)],
            vec![None, None, Some(7), Some(0)],
        ],
        adjm
    );
    for i in 0..v {
        for j in 0..v {
            match adjm[i][j] {
                Some(d) => print!("{}", d),
                None => print!("INF"),
            }
            if j != v - 1 {
                print!(" ");
            }
        }
        print!("\n");
    }
}

#[test]
fn check_negative_cycle() {
    let v = 4;
    let mut adjm: Vec<Vec<Option<i64>>> = vec![
        vec![None, Some(1), Some(5), None],
        vec![None, None, Some(2), Some(4)],
        vec![None, None, None, Some(1)],
        vec![None, None, Some(-7), None],
    ];
    // should be zero not self loop
    for i in 0..v {
        match adjm[i][i] {
            Some(_d) => (),
            None => adjm[i][i] = Some(0),
        }
    }
    warshall_floyd(&mut adjm);

    assert_eq!(
        vec![
            vec![Some(0), Some(1), Some(-3), Some(-2)],
            vec![None, Some(0), Some(-4), Some(-3)],
            vec![None, None, Some(-6), Some(-5)],
            vec![None, None, Some(-13), Some(-12)],
        ],
        adjm
    );
    for i in 0..v {
        print!("vec![",);
        for j in 0..v {
            match adjm[i][j] {
                Some(d) => print!("Some({})", d),
                None => print!("None"),
            }
            if j != v - 1 {
                print!(",");
            }
        }
        print!("],",);
        print!("\n");
    }
}

fn main() {}

//=======

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
