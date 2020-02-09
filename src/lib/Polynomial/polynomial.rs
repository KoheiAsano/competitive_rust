// coefficient trait
trait Coef:
    
    Copy
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
}
impl Coef for i64 {}
impl Coef for u64 {}
impl Coef for f64 {}
#[derive(Eq, PartialEq, Clone)]
struct Polynomial<T: Coef> {
    coef: Vec<T>,
}
// display, debug
impl<T: Coef> std::fmt::Debug for Polynomial<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = format!("{}", self.coef[0]);
        for i in 1..self.coef.len() {
            res = format!("{}+{}(x{})", res, self.coef[i], i);
        }
        write!(f, "{}", res)
    }
}

// constructor
impl<T: Coef> Polynomial<T> {
    fn new() -> Self {
        Polynomial {
            coef: vec![T::default()],
        }
    }
}
impl<T: Coef> From<Vec<T>> for Polynomial<T> {
    fn from(a: Vec<T>) -> Self {
        Polynomial { coef: a.to_vec() }
    }
}

#[test]
fn check_new_from() {
    let p = Polynomial::<i64>::new();
    println!("{:?}", p);
    let v: Vec<i64> = vec![1, 2, 3, 4, 5];
    let p = Polynomial::from(v);
    println!("{:?}", p);
}

// operations these borrow references

impl<T: Coef> std::ops::Add<Polynomial<T>> for Polynomial<T> {
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
impl<T: Coef> std::ops::Sub<Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;

    fn sub(self, rhs: Polynomial<T>) -> Self::Output {
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
impl<T: Coef> std::ops::Mul<Polynomial<T>> for Polynomial<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::fmt::Debug
        + From<i32>
        + Copy
        + Clone,
{
    type Output = Polynomial<T>;

    fn mul(self, rhs: Polynomial<T>) -> Self::Output {
        Polynomial { coef: single_convolution(self.coef, b: rhs.coef) }
    }
}

#[test]
fn check_ops() {}

fn main() {}
