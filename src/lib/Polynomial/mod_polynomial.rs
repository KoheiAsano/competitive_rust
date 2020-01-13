// =========
pub trait ModI:
    Sized
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
    let mut res = mod_conv(&mut a, &mut b);
    res.truncate(d);
    res
}

#[derive(Eq, PartialEq, Clone)]
struct Polynomial<T: ModI> {
    coef: Vec<T>,
}
// display, debug
impl<T: ModI> std::fmt::Debug for Polynomial<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = format!("{}", self.coef[0]);
        for i in 1..self.coef.len() {
            res = format!("{}(x{})+{}", self.coef[i], i, res);
        }
        write!(f, "{}", res)
    }
}

// constructor
impl<T: ModI> Polynomial<T> {
    fn new() -> Self {
        Polynomial {
            coef: vec![T::default()],
        }
    }
}
impl<T: ModI> From<Vec<T>> for Polynomial<T> {
    fn from(a: Vec<T>) -> Self {
        Polynomial { coef: a.to_vec() }
    }
}

#[test]
fn check_new_from() {
    type F = ModInt998244353;
    let p = Polynomial::<F>::new();
    println!("{:?}", p);
    let v: Vec<F> = vec![F::new(1), F::new(1), F::new(1), F::new(1), F::new(5)];
    let p = Polynomial::from(v);
    println!("{:?}", p);
}

// operations these borrow references

impl<T: ModI> std::ops::Add<Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;

    fn add(mut self, mut rhs: Polynomial<T>) -> Self::Output {
        if self.coef.len() < rhs.coef.len() {
            for i in 0..self.coef.len() {
                rhs.coef[i] += self.coef[i];
            }
            rhs
        } else {
            for i in 0..rhs.coef.len() {
                self.coef[i] += rhs.coef[i];
            }
            self
        }
    }
}

// component wise
impl<T: ModI> std::ops::Sub<Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;

    fn sub(mut self, rhs: Polynomial<T>) -> Self::Output {
        if self.coef.len() < rhs.coef.len() {
            for i in 0..self.coef.len() {
                self.coef[i] -= rhs.coef[i];
            }
            self
        } else {
            for i in 0..rhs.coef.len() {
                self.coef[i] -= rhs.coef[i];
            }
            self
        }
    }
}

// dot product
// i*j x j * k
impl<T: ModI> std::ops::Mul<Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;

    fn mul(mut self, mut rhs: Polynomial<T>) -> Self::Output {
        Polynomial {
            coef: single_convolution(&mut self.coef, &mut rhs.coef),
        }
    }
}
impl<T: ModI> std::ops::Div<Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;

    fn div(mut self, rhs: Polynomial<T>) -> Self::Output {
        if self.coef.len() < rhs.coef.len() {
            Polynomial::new()
        } else {
            let n = self.coef.len();
            let m = rhs.coef.len();
            let res_size = n - m + 1;
            let mut res = Polynomial::from(vec![T::default(); res_size]);
            for i in 0..res_size {
                // if self.coef[n - (i + 1)] % rhs.coef[m - 1] != 0 {}
                let b = self.coef[n - (i + 1)] / rhs.coef[m - 1];
                res.coef[res_size - (i + 1)] = b;
                for j in 1..m {
                    self.coef[n - (i + j)] -= b * rhs.coef[m - j];
                }
            }
            res
        }
    }
}
impl<T: ModI> std::ops::Rem<Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;

    fn rem(mut self, rhs: Polynomial<T>) -> Self::Output {
        if self.coef.len() < rhs.coef.len() {
            self
        } else {
            let n = self.coef.len();
            let m = rhs.coef.len();
            let res_size = n - m + 1;
            let mut res = Polynomial::from(vec![T::default(); res_size]);
            for i in 0..res_size {
                let b = self.coef[n - (i + 1)] / rhs.coef[m - 1];
                res.coef[res_size - (i + 1)] = b;
                for j in 1..m + 1 {
                    self.coef[n - (i + j)] -= b * rhs.coef[m - j];
                }
                println!("{:?}", self);
            }
            self
        }
    }
}
#[test]
fn check_ops() {
    type F = ModInt998244353;
    let p = Polynomial::<F>::new();
    println!("{:?}", p);
    let v: Vec<F> = vec![F::new(1), F::new(2), F::new(3), F::new(4)];
    let p = Polynomial::from(v);
    let v: Vec<F> = vec![F::new(1), F::new(2), F::new(4), F::new(8)];
    let q = Polynomial::from(v);
    println!("{:?}", p * q);
    let v: Vec<F> = vec![F::new(1), F::new(5), F::new(4), F::new(1)];
    let p = Polynomial::from(v);
    println!("p={:?}", p);
    let v: Vec<F> = vec![F::new(1), F::new(0), F::new(1)];
    let q = Polynomial::from(v);
    println!("q={:?}", q);
    println!("{:?}", p.clone() / q.clone());
    println!("{:?}", p.clone() % q.clone());
}

fn main() {}
