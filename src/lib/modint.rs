const MOD: u64 = 998_244_353;

#[derive(Clone, Copy)]
struct ModInt(u64);

impl ModInt {
    pub fn new(x: u64) -> ModInt {
        ModInt(x % MOD)
    }

    pub fn pow(mut self, mut n: u64) -> ModInt {
        let mut ret = ModInt::new(1);
        while n > 0 {
            if n & 1 == 1 {
                ret *= self;
            }
            self *= self;
            n >>= 1;
        }
        ret
    }

    pub fn inv(&self) -> ModInt {
        self.pow(MOD - 2)
    }
}


impl std::convert::From<u64> for ModInt {
    fn from(x: u64) -> ModInt {
        ModInt::new(x)
    }
}

// Assign
impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: ModInt) {
        self.0 += rhs.0;
        if self.0 >= MOD {
            self.0 -= MOD;
        };
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: ModInt) {
        self.0 += MOD - rhs.0;
        if self.0 >= MOD {
            self.0 -= MOD;
        };
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: ModInt) {
        self.0 = (self.0 as u64) * (rhs.0 as u64) % (MOD as u64);
    }
}

impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: ModInt) {
        *self *= rhs.inv();
    }
}

// Binary operator
impl std::ops::Add for ModInt {
    type Output = ModInt;
    fn add(mut self, rhs: ModInt) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(mut self, rhs: ModInt) -> Self::Output {
        self -= rhs;
        self
    }
}

impl std::ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(mut self, rhs: ModInt) -> Self::Output {
        self *= rhs;
        self
    }
}

impl std::ops::Div for ModInt {
    type Output = ModInt;
    fn div(mut self, rhs: ModInt) -> Self::Output {
        self /= rhs;
        self
    }
}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::fmt::Debug for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

mod tests {
    use super::*;
    #[test]
    fn check_modint() {
        let a = ModInt::new(1);
        let b = ModInt::new(1);
        println!("{:?}", a + b);
        println!("{:?}", a);
    }

}
