// =========
pub trait ModI:
    Sized
    + PartialEq
    + Copy
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + std::default::Default
    + std::fmt::Display
    + std::fmt::Debug
{
    fn m() -> u64;
    fn new(x: u64) -> Self;
    fn pow(self, n: u64) -> Self;
    fn inv(&self) -> Self;
}
macro_rules! define_modint {
    ($n:ident,$m:expr) => {
        #[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
        struct $n(u64);

        #[allow(dead_code)]
        impl ModI for $n {
            fn m() -> u64 {
                $m
            }
            fn new(x: u64) -> $n {
                $n(x % $m)
            }

            fn pow(self, mut n: u64) -> $n {
                let mut ret = $n::new(1);
                let mut base = self;
                while n > 0 {
                    if n & 1 == 1 {
                        ret *= base;
                    }
                    base *= base;
                    n >>= 1;
                }
                ret
            }

            fn inv(&self) -> $n {
                self.pow($m - 2)
            }
        }

        impl std::default::Default for $n {
            fn default() -> $n {
                $n::new(0u64)
            }
        }

        impl std::convert::From<u64> for $n {
            fn from(x: u64) -> $n {
                $n::new(x)
            }
        }

        // Binary operator
        impl std::ops::Add for $n {
            type Output = $n;
            fn add(self, rhs: $n) -> Self::Output {
                $n::new(self.0 + rhs.0)
            }
        }

        impl std::ops::Sub for $n {
            type Output = $n;
            fn sub(self, rhs: $n) -> Self::Output {
                if self.0 >= rhs.0 {
                    $n::new(self.0 - rhs.0)
                } else {
                    $n::new($m - rhs.0 + self.0)
                }
            }
        }

        impl std::ops::Mul for $n {
            type Output = $n;
            fn mul(self, rhs: $n) -> Self::Output {
                $n::new(self.0 * rhs.0)
            }
        }

        impl std::ops::Div for $n {
            type Output = $n;
            fn div(self, rhs: $n) -> Self::Output {
                $n::new(self.0 / rhs.0)
            }
        }

        // Assign
        impl std::ops::AddAssign for $n {
            fn add_assign(&mut self, rhs: $n) {
                *self = *self + rhs;
            }
        }

        impl std::ops::SubAssign for $n {
            fn sub_assign(&mut self, rhs: $n) {
                *self = *self - rhs;
            }
        }

        impl std::ops::MulAssign for $n {
            fn mul_assign(&mut self, rhs: $n) {
                *self = *self * rhs;
            }
        }

        impl std::ops::DivAssign for $n {
            fn div_assign(&mut self, rhs: $n) {
                *self = *self / rhs;
            }
        }

        impl std::fmt::Display for $n {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl std::fmt::Debug for $n {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
// 10^8 < p < 10^9
// 167772161 = 5*2^25 + 1, 469762049 = 7*2^26 + 1, 998244353 = 119*2^23 + 1
// define_modint!(ModInt167772161, 167772161);
// define_modint!(ModInt469762049, 469762049);
define_modint!(ModInt998244353, 998244353);
fn ntt<T: ModI>(a: &mut [T], n: usize, inv: bool) {
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
        let zeta: T = T::new(3).pow((T::m() - 1) / (2 * b as u64));
        for j in 0..b {
            // 3 is primitive root of proth prime
            // 3 ^ ((m - 1) / (n * j)) is primitive n root's j power
            let e: T = if inv {
                zeta.pow(j as u64).inv()
            } else {
                zeta.pow(j as u64)
            };
            let mut k = 0;
            while k < n {
                let s: T = a[j + k];
                let t: T = a[j + k + b] * e;
                a[j + k] = s + t;
                a[j + k + b] = s - t;
                k += b * 2;
            }
        }
        b *= 2;
    }

    if inv {
        let ni = T::new(n as u64).inv();
        for i in 0..n {
            a[i] *= ni;
        }
    }
}

fn mod_conv<T: ModI>(mut a: &mut [T], mut b: &mut [T]) -> Vec<T> {
    let n = a.len();
    // calc each mod
    ntt(&mut a, n, false);
    ntt(&mut b, n, false);
    let mut c = Vec::with_capacity(n);
    for i in 0..n {
        c.push(a[i] * b[i]);
    }
    ntt(&mut c, n, true);
    c
}

fn single_convolution<T: ModI>(a: &mut [T], b: &mut [T]) -> Vec<T> {
    let d: usize = a.len() + b.len() - 1;
    let n = d.checked_next_power_of_two().unwrap();
    let mut a = a.to_vec();
    a.resize(n, T::new(0));
    let mut b = b.to_vec();
    b.resize(n, T::new(0));
    mod_conv(&mut a, &mut b)
}

fn main() {
    input! {
        n:usize,
        ab: [(u64, u64);n]
    }
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    a.push(F::new(0));
    b.push(F::new(0));
    type F = ModInt998244353;
    for i in 0..n {
        a.push(F::new(ab[i].0));
        b.push(F::new(ab[i].1));
    }
    let res = single_convolution(&mut a, &mut b);
    for i in 1..2 * n + 1 {
        println!("{}", res[i].0);
    }
}
