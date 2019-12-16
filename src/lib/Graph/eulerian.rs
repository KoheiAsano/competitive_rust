// オイラーTrail(路)かCycle()を返す
// - 隣接リストを破壊更新
// - 多重辺は認めない

// ===========
fn directed_hierholder(adjl: &mut Vec<Vec<usize>>, start: usize) -> Option<Vec<usize>> {
    // 全ての(入次数-出次数)=0 一周か
    // か始点-1と終点+1じゃないなら無理の判定
    let mut deg: Vec<isize> = vec![0; adjl.len()];
    for i in 0..adjl.len() {
        for j in 0..adjl[i].len() {
            deg[adjl[i][j]] += 1; //iのoutdeg(を他の点のindegに)
        }
        deg[i] -= adjl[i].len() as isize;
    }
    let mut deg0num = 0;
    for i in 0..adjl.len() {
        if deg[i] == 0 {
            deg0num += 1;
        }
    }
    // 閉路だけほしいときは後半のorをなくす
    if !(adjl.len() - deg0num == 0 || (adjl.len() - deg0num == 2 && deg[start] == -1)) {
        return None;
    }
    // ここまで判定

    //２つのStack(結果のはほぼベクター)
    // res_circuitに確定したvertexを追加する
    let mut res_circuit: Vec<usize> = vec![];
    // これは一旦訪れたものを入れておくStack
    let mut trail_stack: Vec<usize> = vec![];
    trail_stack.push(start);

    let mut cur_v: usize = start;
    while trail_stack.len() > 0 {
        //ifで閉路を作る, elseでその道を確定して、もどって閉路が作れるか確認をする
        if adjl[cur_v].len() > 0 {
            trail_stack.push(cur_v);
            cur_v = adjl[cur_v].pop().unwrap()
        } else {
            res_circuit.push(cur_v);
            cur_v = trail_stack.pop().unwrap();
        }
    }
    // 有向なのでひっくりかえす
    res_circuit.reverse();
    Some(res_circuit)
}

fn undirected_hierholder(adjl: &mut Vec<Vec<usize>>, start: usize) -> Option<Vec<usize>> {
    let mut deg: Vec<isize> = vec![0; adjl.len()];
    for i in 0..adjl.len() {
        deg[i] += adjl[i].len() as isize;
    }
    let mut degevennum = 0;
    for i in 0..adjl.len() {
        if deg[i] % 2 == 0 {
            degevennum += 1;
        }
    }
    // 全て偶数次か
    // 始点と終点だけ奇数次
    // 閉路だけほしいときは後半をなくす
    if !(adjl.len() - degevennum == 0 || (adjl.len() - degevennum == 2 && deg[start] % 2 == 1)) {
        println!("aa");
        return None;
    }
    // ここまで判定

    //２つのStack(結果のはほぼベクター)
    // res_circuitに確定したvertexを追加する
    let mut res_circuit: Vec<usize> = vec![];
    // これは一旦訪れたものを入れておくStack
    let mut trail_stack: Vec<usize> = vec![];
    trail_stack.push(start);

    let mut cur_v: usize = start;
    let mut next_v: usize;
    while trail_stack.len() > 0 {
        //ifで閉路を作る, elseでその道を確定して、もどって閉路が作れるか確認をする
        if adjl[cur_v].len() > 0 {
            trail_stack.push(cur_v);
            // 辺を見つけて、捨てて、cur_vを移す
            next_v = adjl[cur_v].pop().unwrap();
            for i in 0..adjl[next_v].len() {
                if adjl[next_v][i] == cur_v {
                    adjl[next_v].remove(i);
                    break;
                }
            }
            cur_v = next_v;
        } else {
            res_circuit.push(cur_v);
            cur_v = trail_stack.pop().unwrap();
        }
    }
    // 有向なのでひっくりかえす
    res_circuit.reverse();
    Some(res_circuit)
}
// ===========
mod tests {
    use super::*;
    #[test]
    fn check_directed_list() {
        // eulerian cycle
        let mut adjl = vec![vec![1], vec![2], vec![3], vec![4], vec![0]];
        assert_eq!(
            directed_hierholder(&mut adjl, 4).unwrap(),
            vec![4, 0, 1, 2, 3, 4]
        );

        // eulerian trail
        let mut adjl = vec![vec![1], vec![2], vec![3], vec![4], vec![0, 3]];

        assert_eq!(
            directed_hierholder(&mut adjl, 4).unwrap(),
            vec![4, 3, 4, 0, 1, 2, 3]
        );
    }

    #[test]
    fn check_undirected_list() {
        // eulerian cycle
        let mut adjl = vec![vec![4, 1], vec![0, 2], vec![1, 3], vec![2, 4], vec![3, 0]];
        println!("{:?}", undirected_hierholder(&mut adjl, 0).unwrap());
        // assert_eq!

        // eulerian trail
        let mut adjl = vec![
            vec![4, 1, 2],
            vec![0, 2],
            vec![0, 1, 3],
            vec![2, 4],
            vec![0, 3],
        ];
        println!("{:?}", undirected_hierholder(&mut adjl, 0).unwrap());
    }
}
