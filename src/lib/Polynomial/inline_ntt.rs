/*
No mod int for embedding
*/

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
// mod mの体におけるaの逆元
fn mod_inv(a: i64, m: i64) -> i64 {
    mod_pow(a, m - 2, m)
}

// 167772161 = 5*2^25 + 1, 469762049 = 7*2^26 + 1, 998244353 = 119*2^23 + 1,
// const MOD: [i64; 3] = [167772161, 469762049, 998244353];
// inplace, non-recursive
fn ntt(a: &mut Vec<i64>, mod_root: (i64, i64), inv: bool) {
    let (m, r) = mod_root;
    let n = a.len();
    // h = log2(n)
    let h = {
        let mut i = 0;
        while 1 << i != n {
            i += 1;
        }
        i
    };
    let mut j: usize;
    for i in 0..n {
        j = 0;
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
        let zeta: i64 = mod_pow(r, (m - 1) / (2 * b as i64), m);
        for j in 0..b {
            // 3 is primitive root of proth prime
            // 3 ^ ((m - 1) / (n * j)) is primitive n root's j power
            let e: i64 = if inv {
                mod_inv(mod_pow(zeta, j as i64, m), m)
            } else {
                mod_pow(zeta, j as i64, m)
            };
            let mut k = 0;
            while k < n {
                let s: i64 = a[j + k];
                let t: i64 = a[j + k + b] * e % m;
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
            if a[i] < 0 {
                a[i] += m;
            }
        }
    }
}
// size is assumed to 2 power
fn mod_conv(a: &mut Vec<i64>, b: &mut Vec<i64>, mod_root: (i64, i64)) -> Vec<i64> {
    let n = a.len();
    let mut f: Vec<i64> = Vec::with_capacity(n);
    ntt(a, mod_root, false);
    ntt(b, mod_root, false);
    for i in 0..n {
        f.push(a[i] * b[i] % mod_root.0);
    }
    ntt(&mut f, mod_root, true);
    f
}

fn single_convolution(a: &mut Vec<i64>, b: &mut Vec<i64>) -> Vec<i64> {
    const MOD_ROOT: (i64, i64) = (167772161, 3);
    let d: usize = a.len() + b.len() - 1;
    let n = d.checked_next_power_of_two().unwrap();
    a.resize(n, 0);
    b.resize(n, 0);
    mod_conv(a, b, MOD_ROOT)
}

// mr ... 互いに素, m ... 答えを求めたい体での素数
fn garner(mr: &mut Vec<(i64, i64)>, m: i64) -> i64 {
    mr.push((m, 0));
    // coef... mixed radixの係数, constants... 前まで求めた係数
    let mut coef: Vec<i64> = vec![1; mr.len()];
    let mut constants: Vec<i64> = vec![0; mr.len()];
    for i in 0..mr.len() - 1 {
        let mut v: i64 = (mr[i].1 - constants[i]) * mod_inv(coef[i], mr[i].0) % mr[i].0;
        if v < 0 {
            v += mr[i].0;
        }
        for j in i + 1..mr.len() {
            constants[j] += coef[j] * v;
            constants[j] %= mr[j].0;
            coef[j] *= mr[i].0;
            coef[j] %= mr[j].0;
        }
    }
    constants[mr.len() - 1]
}
// convolution for more bigger number
fn convolution(a: &mut Vec<i64>, b: &mut Vec<i64>) -> Vec<i64> {
    const MOD_ROOT: [(i64, i64); 3] = [(167772161, 3), (469762049, 3), (998244353, 3)];
    // length is degree + 1, result is a-deg + b-deg degree
    let d: usize = a.len() + b.len() - 1;
    let n = d.checked_next_power_of_two().unwrap();
    a.resize(n, 0);
    b.resize(n, 0);
    let mut candmr: Vec<Vec<(i64, i64)>> = vec![];
    for mod_root in MOD_ROOT.iter() {
        candmr.push(
            mod_conv(&mut a.clone(), &mut b.clone(), *mod_root)
                .iter()
                .map(|e| (mod_root.0, *e))
                .collect::<Vec<(i64, i64)>>(),
        );
    }
    // CRT
    let mut ans: Vec<i64> = vec![];
    for i in 0..d {
        let mut mr: Vec<(i64, i64)> = vec![];
        for j in 0..MOD_ROOT.len() {
            mr.push(candmr[j][i]);
        }
        ans.push(garner(&mut mr, 10000000007));
    }
    ans
}
