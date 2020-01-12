/*
value are supposed to be able to convert from u32
*/

const N: usize = 3;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Matrix<T>
where
    T: Copy + Clone,
{
    val: [[T; N]; N],
}

// Debug Display
impl<T> std::fmt::Debug for Matrix<T>
where
    T: std::fmt::Debug + Copy + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::new();
        for i in 0..N {
            res = format!("{}{:?}\n", res, self.val[i]);
        }
        write!(f, "{}", res)
    }
}

// constructors
#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: From<i32> + Copy + Clone,
{
    fn i() -> Self {
        let mut val: [[T; N]; N] = [[T::from(0i32); N]; N];
        for i in 0..N {
            val[i][i] = T::from(1i32);
        }
        Matrix { val: val }
    }
    fn zero() -> Self {
        let val: [[T; N]; N] = [[T::from(0i32); N]; N];
        Matrix { val: val }
    }
    // generate permutation matrix
    fn permutation(pi: usize, pj: usize) -> Self {
        let mut val: [[T; N]; N] = [[T::from(0i32); N]; N];
        for i in 0..N {
            if i == pi {
                val[i][pj] = T::from(1i32);
            } else if i == pj {
                val[i][pi] = T::from(1i32);
            } else {
                val[i][i] = T::from(1i32);
            }
        }
        Matrix { val: val }
    }
}

#[test]
fn check_gen() {
    println!("{:?}", Matrix::<i64>::i());
    let m = [[1, 2, 34], [1, 2, 34], [1, 2, 34]];
    println!("{:?}", Matrix::from(m));
}

// methods
#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: From<i32> + Copy + Clone,
{
    fn transpose(&self) -> Self {
        let mut tval: [[T; N]; N] = [[T::from(0i32); N]; N];
        for i in 0..N {
            for j in i..N {
                tval[j][i] = self.val[i][j];
                tval[i][j] = self.val[j][i];
            }
        }
        Matrix { val: tval }
    }
}

#[test]
fn check_permutation() {
    let m: Matrix<i64> = Matrix::from([[3, 2, 1], [1, 4, 5], [1, 2, 3]]);
    let p12: Matrix<i64> = Matrix::permutation(1, 2);
    println!("m=\n{:?}", m);
    println!("p=\n{:?}", p12);
    // 右から掛けると列交換
    println!("m*p=\n{:?}", m * p12);
    let m: Matrix<i64> = Matrix::from([[3, 2, 1], [1, 4, 5], [1, 2, 3]]);
    let p12: Matrix<i64> = Matrix::permutation(1, 2);
    // 左から掛けると行交換
    println!("p*m=\n{:?}", p12 * m);
}

impl<T: Copy + Clone> From<[[T; N]; N]> for Matrix<T> {
    fn from(m: [[T; N]; N]) -> Self {
        Matrix { val: m }
    }
}

// operations
impl<T> std::ops::Neg for Matrix<T>
where
    T: std::ops::Neg<Output = T> + From<i32> + Copy + Clone,
{
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        let mut res = [[T::from(0i32); N]; N];
        for i in 0..N {
            for j in 0..N {
                res[i][j] = -self.val[i][j];
            }
        }
        Matrix { val: res }
    }
}

// component wise
impl<T> std::ops::Add<Matrix<T>> for Matrix<T>
where
    T: std::ops::Add<Output = T> + std::fmt::Debug + From<i32> + Copy + Clone,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        let mut res = [[T::from(0i32); N]; N];
        for i in 0..N {
            for j in 0..N {
                res[i][j] = self.val[i][j] + rhs.val[i][j];
            }
        }
        Matrix { val: res }
    }
}

#[test]
fn check_ops() {
    println!("{:?}", Matrix::<i64>::i() + Matrix::<i64>::i());
    println!("{:?}", Matrix::<i64>::i() - Matrix::<i64>::i());
    println!("{:?}", Matrix::<i64>::i() * Matrix::<i64>::i());
    let m1 = Matrix::from([[3, 2, 1], [1, 4, 5], [1, 2, 3]]);
    let m2 = Matrix::from([[1, 2, 3], [3, 2, 1], [1, 3, 2]]);
    println!("{:?}", m1 * m2);
    println!("{:?}", m1 * m2);
}

// component wise
impl<T> std::ops::Sub<Matrix<T>> for Matrix<T>
where
    T: std::ops::Sub<Output = T> + std::fmt::Debug + From<i32> + Copy + Clone,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        let mut res = [[T::from(0i32); N]; N];
        for i in 0..N {
            for j in 0..N {
                res[i][j] = self.val[i][j] - rhs.val[i][j];
            }
        }
        Matrix { val: res }
    }
}

// dot product
// i*j x j * k
impl<T> std::ops::Mul<Matrix<T>> for Matrix<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::fmt::Debug
        + From<i32>
        + Copy
        + Clone,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        let mut res = [[T::from(0i32); N]; N];
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    res[i][k] = res[i][k] + (self.val[i][j] * rhs.val[j][k]);
                }
            }
        }
        Matrix { val: res }
    }
}

fn main() {}

#[test]
fn check_complex() {
    let i = Complex::i();
    let cm: Matrix<Complex> = Matrix::from([
        [i, Complex::zero(), Complex::zero()],
        [Complex::zero(), i, Complex::zero()],
        [Complex::zero(), Complex::zero(), i],
    ]);
    println!("{:?}", cm * cm);
}
