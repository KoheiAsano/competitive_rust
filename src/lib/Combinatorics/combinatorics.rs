/*
MAXN, MODを定数宣言して
let c = Comb::fact_new();
*/
//本当はfactを定数で前計算したい。
//定数内でループが書けないので困った
//一旦その周りを構造体にしてしまおう

// ==============
const MAXN: usize = 10010;
const MOD: usize = 1000000007;

struct Comb {
    fact: [usize; MAXN],
    finv: [usize; MAXN],
}
// 階乗・順列・組み合わせ・重複組合せ
// modとmaxnは引数に持たせないことにした(配列の初期化ができない)
impl Comb {
    // これをメンバーに持たせるのは悩みどころさん
    fn p_mod(base: usize, exp: usize) -> usize {
        if exp != 0 {
            if (exp & 1) == 1 {
                Comb::p_mod(base * base % MOD, exp / 2) * (base) % MOD
            } else {
                Comb::p_mod(base * base % MOD, exp / 2)
            }
        } else {
            1
        }
    }

    fn fact_new() -> Self {
        let mut fact: [usize; MAXN] = [0; MAXN];
        fact[0] = 1;
        for i in 1..MAXN {
            fact[i] = (fact[i - 1] * i) % MOD;
        }
        let mut finv: [usize; MAXN] = [0; MAXN];
        finv[MAXN - 1] = Comb::p_mod(fact[MAXN - 1], MOD - 2);
        for i in (0..(MAXN - 1)).rev() {
            finv[i] = (finv[i + 1] * (i + 1)) % MOD;
        }
        Comb {
            fact: fact,
            finv: finv,
        }
    }
    fn nPr(&self, n: usize, r: usize) -> usize {
        if n < r || r == 0 {
            0
        } else if r == 0 {
            1
        } else {
            self.fact[n] * self.finv[n - r] % MOD
        }
    }
    fn nCr(&self, n: usize, r: usize) -> usize {
        if n < r {
            0
        } else if r == 0 {
            1
        } else {
            self.fact[n] * self.finv[r] % MOD * self.finv[n - r] % MOD
        }
    }
    fn nHr(&self, n: usize, r: usize) -> usize {
        if n == 0 && r == 0 {
            1
        } else {
            self.nCr(n + r - 1, r)
        }
    }
}
// ==============

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_mod() {
        assert_eq!(Comb.p_mod(2, 8, MOD), 256);
        assert_eq!(Comb.p_mod(10, 7, MOD), 10000000);
        assert_eq!(Comb.p_mod(10, 10, MOD), 999999937);
    }

    #[test]
    fn test_fact() {
        // factとfinvは可逆の関係
        let c = Comb::fact_new();
        for i in 0..MAXN {
            assert_eq!(c.fact[i] * c.finv[i] % MOD, 1);
        }
    }

    #[test]
    fn test_combinatorics() {
        let c = Comb::fact_new();
        assert_eq!(c.nCr(5, 3), 10);
        assert_eq!(c.nPr(5, 3), 60);
        // 5H3は5種類から3つ重複を許してとる
        // ||||ooo
        assert_eq!(c.nHr(5, 3), 35);
        assert_eq!(c.nHr(3, 4), 15);
        assert_eq!(c.nHr(4, 6), 84);
    }
}
