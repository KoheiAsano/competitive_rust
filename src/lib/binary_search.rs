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
