
fn norm(p: (f64, f64)) -> f64 {
    (p.0.powf(2.0) + p.1.powf(2.0)).sqrt()
}

fn sin(rad: f64) -> f64 {
    // 定数の宣言

    use std::f64::consts::PI;
    // 角度の誤差
    let eps = 1e-18;
    let mut left = (0.0, 1.0);
    let mut left_a = PI / 2.0;
    let mut right = (1.0, 0.0);
    let mut right_a = 0.0;
    // 符号の計算
    // 正にする
    let mut rad = rad % (2.0 * PI);
    if rad < 0.0 {
        rad += 2.0 * PI;
    }

    let sign: f64 = if rad.abs() > PI { -1.0 } else { 1.0 };
    // 正の角度に
    // 第一象限にする
    if PI / 2.0 > rad {
        rad = rad;
    } else if PI > rad {
        rad = PI - rad;
    } else if 3.0 * PI / 2.0 >= rad {
        rad = rad - PI;
    } else {
        rad = 2.0 * PI - rad;
    }
    // 値を二分探索
    // 正規化をする
    let mut val: (f64, f64) = (
        (left.0 + right.0) / 2.0f64.sqrt(),
        (left.1 + right.1) / 2.0f64.sqrt(),
    );

    let mut angle = (left_a + right_a) / 2.0;
    let mut n: f64;
    while (rad - angle).abs() > eps {
        if rad > angle {
            right = val;
            right_a = angle;
        } else {
            left = val;
            left_a = angle;
        }
        val = ((left.0 + right.0), (left.1 + right.1));
        n = norm(val);
        val = (val.0 / n, val.1 / n);
        angle = (left_a + right_a) / 2.0;
    }
    sign * val.1
}