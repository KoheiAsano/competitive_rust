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
define_modint!(ModInt998244353, 998244353);
define_modint!(ModInt167772161, 167772161);
define_modint!(ModInt469762049, 469762049);

mod tests {
    use super::*;
    #[test]
    fn check_modint() {
        let mut a = ModInt998244353::new(1);
        let b = ModInt998244353::new(1);
        println!("{:?}", a);
        println!("{:?}", a.0);
    }
}

fn main() {}

// for ntt trait
pub trait ModI: Sized {
    fn m() -> u64;
    fn new(x: u64) -> Self;
    fn pow(self, mut n: u64) -> Self;
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
            // when m is not prime
            // pub fn inv2(&self) -> $n {
            //     let mut ab = (self, $n::new(0));
            //     let mut uv = ($n::new(1), $n::new(0));
            //     let mut t: i64;
            //     while ab.1 != 0 {
            //         t = ab.0 / ab.1;
            //         ab = (ab.1, ab.0 - t * ab.1);
            //         uv = (uv.1, uv.0 - t * uv.1);
            //     }
            //     if ab.0 != 1 {
            //         panic!("{} and {} are not coprime g={}", a, m, ab.0);
            //     }
            //     let inv = uv.0 % m as i64;
            //     if inv < 0 {
            //         inv + m as i64
            //     } else {
            //         inv as i64
            //     }
            // }
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
