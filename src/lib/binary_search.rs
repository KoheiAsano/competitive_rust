fn lower_bound<T: std::cmp::Ord>(arr: &Vec<T>, b: T) -> usize {
    let (mut lt, mut ge) = (0, arr.len());
    // lt=-1にすると型強制しなきゃなのでここで確かめる
    if arr[lt] >= b {
        return 0;
    }
    while lt + 1 < ge {
        let m = (lt + ge) / 2;
        if arr[m] < b {
            lt = m;
        } else if arr[m] > b {
            ge = m;
        } else {
            return m;
        }
    }
    ge
}

fn is_ok(u: usize) -> bool {
    if u > 6 {
        false
    } else {
        true
    }
}

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    if is_ok(a[0]) {
        let mut ok = 0;
        let mut ng = 10;
        let mut cur = (ng + ok) / 2;
        while (ng - ok) != 1 {
            if is_ok(a[cur]) {
                ok = cur;
            } else {
                ng = cur;
            }
            cur = (ng + ok) / 2;
        }
        println!("{:?}", ok);
        println!("{:?}", ng);
        println!("{:?}", a[ok]);
    } else {
        // 無理な処理
    }
}
