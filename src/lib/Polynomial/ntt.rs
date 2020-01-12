// mod mの体におけるaの逆元
fn mod_inv(a: i64, m: i64) -> i64 {
    let mut ab = (a as i64, m as i64);
    let mut uv = (1, 0);
    let mut t: i64;
    while ab.1 != 0 {
        t = ab.0 / ab.1;
        ab = (ab.1, ab.0 - t * ab.1);
        uv = (uv.1, uv.0 - t * uv.1);
    }
    if ab.0 != 1 {
        panic!("{} and {} are not coprime g={}", a, m, ab.0);
    }
    let inv = uv.0 % m as i64;
    if inv < 0 {
        (inv + m as i64) as i64
    } else {
        inv as i64
    }
}
fn mod_pow(mut a: i64, mut n: i64, m: i64) -> i64 {
    let mut ret = 1;
    while n > 0 {
        if n & 1 == 1 {
            ret *= a;
            ret %= m;
        }
        a *= a;
        a %= m;
        n >>= 1;
    }
    ret
}

// mr ... 互いに素
fn garner(mr: &mut Vec<(i64, i64)>, m: i64) -> i64 {
    mr.push((m, 0));
    // coef... mixed radixの係数, constants... 前まで求めた係数
    let mut coef: Vec<i64> = vec![1; mr.len()];
    let mut constants: Vec<i64> = vec![0; mr.len()];
    for i in 0..mr.len() - 1 {
        let mut v: i64 = (mr[i].1 - constants[i]) % mr[i].0 * mod_inv(coef[i], mr[i].0) % mr[i].0;
        if v < 0 {
            v += mr[i].0;
        }
        for j in i + 1..mr.len() {
            // とりあえずwrapping_mulをしてみる
            // constants[j] += coef[j] * v;
            constants[j] += coef[j].wrapping_mul(v);
            constants[j] %= mr[j].0;
            // coef[j] *= mr[i].0;
            coef[j] = coef[j].wrapping_mul(mr[i].0);
            coef[j] %= mr[j].0;
        }
    }
    constants[mr.len() - 1]
}

// 167772161 = 5*2^25 + 1, 469762049 = 7*2^26 + 1, 998244353 = 119*2^23 + 1,
const MOD: [i64; 3] = [167772161, 469762049, 998244353];
fn ntt(mut a: Vec<i64>, n: usize, m: i64, inv: bool) -> Vec<i64> {
    if n == 1 {
        return a;
    }
    // h = log2(n)
    let h = {
        let mut i = 0;
        while 1 << i != n {
            i += 1;
        }
        i
    };
    for i in 0..n {
        let mut j: usize = 0;
        for k in 0..h {
            // (i >> k & 1)はiのk桁目のbit
            // (h - 1 - k)は全体をh-bitとしてk桁目の反対の位置
            j |= (i >> k & 1) << (h - 1 - k);
        }
        // はじめの一回だけひっくりかえす
        if i < j {
            a.swap(i, j)
        };
    }
    // バタフライ演算
    let mut b = 1;
    while b < n {
        for j in 0..b {
            // 3 is primitive root of proth prime
            // 3 ^ ((m - 1) / (n * j)) is primitive n root's j power
            let zeta: i64 = if inv {
                mod_inv(mod_pow(3, (m - 1) / (2 * b as i64) * j as i64, m), m)
            } else {
                mod_pow(3, (m - 1) / (2 * b as i64) * j as i64, m)
            };
            let mut k = 0;
            while k < n {
                let s: i64 = a[j + k];
                let t: i64 = a[j + k + b] * zeta % m;
                a[j + k] = (s + t) % m;
                a[j + k + b] = (s - t) % m;
                k += b * 2;
            }
        }
        b *= 2;
    }

    if inv {
        for i in 0..n {
            a[i] *= mod_inv(n as i64, m);
            a[i] %= m;
        }
    }
    return a;
}

fn multiply(mut a: Vec<i64>, mut b: Vec<i64>) -> Vec<i64> {
    // length is degree + 1, result is a-deg + b-deg degree
    let d: usize = a.len() + b.len() - 1;
    let n = d.checked_next_power_of_two().unwrap();
    a.resize(n, 0);
    b.resize(n, 0);
    let a_copy = a.clone();
    let b_copy = b.clone();

    // each field solutions
    let mut cand = vec![];
    // calc each mod
    for m in MOD.iter() {
        let mut a = a_copy.clone();
        let mut b = b_copy.clone();
        a = ntt(a, n, *m, false);
        b = ntt(b, n, *m, false);
        let mut f: Vec<i64> = Vec::with_capacity(n);
        for i in 0..n {
            f.push(a[i] * b[i] % m);
        }
        f = ntt(f, n, *m, true);
        f.truncate(d);
        cand.push(f);
    }

    // recalc
    let mut ans: Vec<i64> = vec![];
    for j in 0..d {
        let mut mr: Vec<(i64, i64)> = vec![];
        for i in 0..MOD.len() {
            mr.push((MOD[i], cand[i][j]));
        }
        ans.push(garner(&mut mr, std::i64::MAX));
    }
    ans
}

fn main() {}

mod tests {
    use super::*;
    #[test]
    fn check_mod_pow() {
        println!("{:?}", mod_pow(3, 4, MOD[1]));
    }
    #[test]
    fn check_multiply() {
        let a = vec![1, 2];
        let b = vec![1, 2];
        let ans = multiply(a, b);
        for i in 0..ans.len() {
            println!("ans[{}]{:?}", i, ans[i]);
        }
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 2, 4, 8];
        let ans = multiply(a, b);
        for i in 0..ans.len() {
            println!("ans[{}]{:?}", i, ans[i]);
        }
    }
    #[test]
    fn check_invmod() {
        println!("{:?}", std::i64::MAX);
        let a = 123456789;
        println!("{:?}", mod_inv(a, MOD[0] as i64) * a % MOD[0] as i64);
        println!("{:?}", mod_inv(3, 14));
        println!("{:?}", mod_inv(3, 115) * 3 % 115);
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
        let mut mr = vec![(15, 1), (17, 0), (31, 0)];
        let ans = garner(&mut mr, 15 * 17 * 31);
        println!("{:?}", 15 * 17 * 31);
        println!("{:?}", ans);
        assert_eq!(ans % 15, 1);
        assert_eq!(ans % 17, 0);
        assert_eq!(ans % 31, 0);
    }

}
