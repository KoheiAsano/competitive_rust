// =========
use std::ops;
#[derive(Clone, Copy, Debug)]
struct Complex {
    re: f64,
    im: f64,
}
impl PartialEq for Complex {
    fn eq(&self, other: &Complex) -> bool {
        (self.re - other.re).abs() < 10e-9 && (self.im - other.im).abs() < 10e-9
    }
}
impl ops::Add<Complex> for Complex {
    type Output = Self;
    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
impl ops::AddAssign<Complex> for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        *self = *self + rhs;
    }
}

impl ops::Mul<Complex> for Complex {
    type Output = Self;
    fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}
impl ops::MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, rhs: Complex) {
        *self = *self * rhs;
    }
}

impl ops::Div<Complex> for Complex {
    type Output = Self;
    fn div(self, rhs: Complex) -> Self::Output {
        let denomi = self.im.powf(2.0) + rhs.im.powf(2.0);
        Complex {
            re: (self.re * rhs.re + self.im * rhs.im) / denomi,
            im: (self.re * rhs.im + self.im * rhs.re) / denomi,
        }
    }
}
impl ops::DivAssign<Complex> for Complex {
    fn div_assign(&mut self, rhs: Complex) {
        *self = *self / rhs;
    }
}

macro_rules! impl_ops {
    ($I:ty) => {
        impl ops::Add<$I> for Complex {
            type Output = Self;
            fn add(self, rhs: $I) -> Self::Output {
                Complex {
                    re: self.re + rhs as f64,
                    im: self.im,
                }
            }
        }
        impl ops::AddAssign<$I> for Complex {
            fn add_assign(&mut self, rhs: $I) {
                *self = *self + rhs as f64;
            }
        }

        impl ops::Mul<$I> for Complex {
            type Output = Self;
            fn mul(self, rhs: $I) -> Self::Output {
                Complex {
                    re: self.re * rhs as f64,
                    im: self.im * rhs as f64,
                }
            }
        }
        impl ops::MulAssign<$I> for Complex {
            fn mul_assign(&mut self, rhs: $I) {
                *self = *self * rhs as f64;
            }
        }

        impl ops::Div<$I> for Complex {
            type Output = Self;
            fn div(self, rhs: $I) -> Self::Output {
                Complex {
                    re: self.re / rhs as f64,
                    im: self.im / rhs as f64,
                }
            }
        }
        impl ops::DivAssign<$I> for Complex {
            fn div_assign(&mut self, rhs: $I) {
                *self = *self / rhs;
            }
        }
    };
}
impl_ops!(f64);
impl_ops!(usize);

impl Complex {
    fn new(re: f64, im: f64) -> Self {
        Complex { re: re, im: im }
    }
    fn zero() -> Self {
        Complex { re: 0.0, im: 0.0 }
    }
    fn i() -> Self {
        Complex { re: 0.0, im: 1.0 }
    }
    fn root(n: usize) -> Self {
        Complex {
            re: (2.0 * std::f64::consts::PI / n as f64).cos(),
            im: (2.0 * std::f64::consts::PI / n as f64).sin(),
        }
    }
    fn pow(self, mut n: usize) -> Self {
        let mut ret = Complex::new(1.0, 0.0);
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
    fn conj(self) -> Self {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }
}

fn dft(mut a: Vec<Complex>, n: usize, inv: bool) -> Vec<Complex> {
    if n == 1 {
        return a;
    }
    let mut f: Vec<Complex> = vec![];
    let mut g: Vec<Complex> = vec![];
    for i in 0..n / 2 {
        f.push(a[2 * i]);
        g.push(a[2 * i + 1]);
    }
    f = dft(f, n / 2, inv);
    g = dft(g, n / 2, inv);

    let zeta: Complex = if inv {
        Complex::root(n).conj()
    } else {
        Complex::root(n)
    };
    let mut p = Complex::new(1.0, 0.0);
    for i in 0..n {
        a[i] = f[i % (n / 2)] + p * g[i % (n / 2)];
        p *= zeta;
    }
    return a;
}

fn idft(mut a: Vec<Complex>, n: usize) -> Vec<Complex> {
    a = dft(a, n, true);
    for i in 0..n {
        a[i] /= n;
    }
    a
}

fn multiply(mut a: Vec<Complex>, mut b: Vec<Complex>) -> Vec<Complex> {
    // length is degree + 1, result is a-deg + b-deg degree
    let d: usize = a.len() + b.len() - 1;
    let n = d.checked_next_power_of_two().unwrap();
    a.resize(n, Complex::zero());
    b.resize(n, Complex::zero());

    a = dft(a, n, false);
    b = dft(b, n, false);
    let mut f: Vec<Complex> = vec![];
    for i in 0..n {
        f.push(a[i] * b[i]);
    }
    f = idft(f, n);
    f.truncate(d);
    f
}

fn main() {}

mod tests {
    use super::*;
    #[test]
    fn check_multiply() {
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 2, 4, 8];
        let a: Vec<Complex> = a.iter().map(|e| Complex::new(*e as f64, 0.0)).collect();
        let b: Vec<Complex> = b.iter().map(|e| Complex::new(*e as f64, 0.0)).collect();
        let ans = multiply(a, b);
        for i in 0..ans.len() {
            if ans[i].re > 0.0 {
                println!("{}", (ans[i].re + 0.5) as i64);
            } else {
                println!("{}", (ans[i].re - 0.5) as i64);
            }
        }
    }

}
