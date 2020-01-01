// mod mの体におけるaの逆元
// Genericで書けたらいいけど力が足りない
fn mod_inv<T>(a: T, m: T) -> T
where
    T: std::marker::Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::SubAssign
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::default::Default
        + PartialOrd<T>
        + PartialEq<T>,
    i64: From<T>,
{
    let mut ab = (i64::from(a), i64::from(m));
    let mut uv = (1, 0);
    let mut t: i64;
    while ab.1 != 0 {
        t = ab.0 / ab.1;
        ab = (ab.1, ab.0 - t * ab.1);
        uv = (uv.1, uv.0 - t * uv.1);
    }
    let inv = uv.0 % i64::from(m);
    if inv < 0 {
        inv + m
    } else {
        inv
    }
}

// mod mの体におけるaの逆元
macro_rules! impl_modinv {
    ($U:ty) => {
        fn mod_inv(a: $U, m: $U) -> $U {
            let mut ab = (a as i64, m as i64);
            let mut uv = (1, 0);
            let mut t: i64;
            while ab.1 != 0 {
                t = ab.0 / ab.1;
                ab = (ab.1, ab.0 - t * ab.1);
                uv = (uv.1, uv.0 - t * uv.1);
            }
            let inv = uv.0 % m as i64;
            if inv < 0 {
                (inv + m as i64) as $U
            } else {
                inv as $U
            }
        }
    };
}

impl_modinv!(u64);
// mr ... 素数,
fn garner(mr: &mut Vec<(u64, u64)>, m: u64) -> u64 {
    mr.push((m, 0));
    // coef... mixed radixの係数, constants... 前まで求めた係数
    let mut coef: Vec<u64> = vec![1; mr.len()];
    let mut constants: Vec<u64> = vec![0; mr.len()];
    for i in 0..mr.len() - 1 {
        // + m はunderflowをさけるため
        let v: u64 = (mr[i].1 + m - constants[i]) * mod_inv(coef[i], mr[i].0) % mr[i].0;
        for j in i + 1..mr.len() {
            constants[j] += coef[j] * v;
            constants[j] %= mr[j].0;
            coef[j] *= mr[i].0;
            coef[j] %= mr[j].0;
        }
    }
    constants[mr.len() - 1]
}


mod tests {
    use super::*;
    #[test]
    fn check_invmod() {
        let a = 123456789;
        println!("{:?}", mod_inv(a, MOD as u64) * a % MOD as u64);
    }

    #[test]
    fn check_gardner() {
        let mut mr = vec![(5, 4), (7, 1), (11, 2)];
        let ans = garner(&mut mr, 385);
        assert_eq!(ans % 5, 4);
        assert_eq!(ans % 7, 1);
        assert_eq!(ans % 11, 2);
        let mut mr = vec![(15, 1), (17, 2), (31, 3)];
        let ans = garner(&mut mr, 15 * 17 * 31);
        assert_eq!(ans % 15, 1);
        assert_eq!(ans % 17, 2);
        assert_eq!(ans % 31, 3);
    }

}
