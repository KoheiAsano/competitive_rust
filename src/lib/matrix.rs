/*
TODO: change vector to array
*/

#[derive(Clone, Eq, PartialEq)]
struct Matrix<T>
where
    T: Copy + Clone,
{
    val: Vec<Vec<T>>,
    size: (usize, usize),
}

impl<T> std::fmt::Debug for Matrix<T>
where
    T: std::fmt::Debug + Copy + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::new();
        for i in 0..self.size.0 {
            res = format!("{}{:?}\n", res, self.val[i]);
        }
        write!(f, "{}", res)
    }
}

// constructors
impl<T> Matrix<T>
where
    T: From<u32> + Copy + Clone,
{
    fn i(n: usize) -> Self {
        let mut val: Vec<Vec<T>> = vec![];
        for i in 0..n {
            let mut row = vec![T::from(0u32); n];
            row[i] = T::from(1u32);
            val.push(row);
        }
        Matrix {
            val: val,
            size: (n, n),
        }
    }
    fn zero(n: usize) -> Self {
        let val: Vec<Vec<T>> = vec![vec![T::from(0u32); n]; n];
        Matrix {
            val: val,
            size: (n, n),
        }
    }
    // generate permutation matrix
    fn permutation(n: usize, pi: usize, pj: usize) -> Self {
        let mut val: Vec<Vec<T>> = vec![vec![T::from(0u32); n]; n];
        for i in 0..n {
            if i == pi {
                val[i][pj] = T::from(1u32);
            } else if i == pj {
                val[i][pi] = T::from(1u32);
            } else {
                val[i][i] = T::from(1u32);
            }
        }
        Matrix {
            val: val,
            size: (n, n),
        }
    }
}

// methods
impl<T> Matrix<T>
where
    T: std::default::Default + Copy + Clone,
{
    fn transpose(&self) -> Self {
        let mut res = vec![vec![T::default(); self.size.0]; self.size.1];
        for i in 0..self.size.1 {
            for j in 0..self.size.0 {
                res[j][i] = self.val[i][j];
            }
        }
        Matrix {
            val: res,
            size: (self.size.1, self.size.0),
        }
    }
}

#[test]
fn check_permutation() {
    let p34: Matrix<i64> = Matrix::permutation(5, 3, 4);
    println!("{:?}", p34);
    let p14: Matrix<i64> = Matrix::permutation(5, 1, 4);
    println!("{:?}", p14);
    println!("{:?}", p14.transpose());
    let p134 = p34 * p14;
    println!("{:?}", p134);
    println!("{:?}", p134.transpose());
    let p134_inv = p134.transpose();
    println!("{:?}", p134 * p134_inv);
}

impl<T: Copy + Clone> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(m: Vec<Vec<T>>) -> Self {
        let s = (m.len(), m[0].len());
        // temporaliry trust its size
        Matrix { val: m, size: s }
    }
}

// operations
impl<T> std::ops::Neg for Matrix<T>
where
    T: std::ops::Neg<Output = T> + std::default::Default + Copy + Clone,
{
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        let mut res = vec![vec![T::default(); self.size.1]; self.size.0];
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                res[i][j] = -self.val[i][j];
            }
        }
        Matrix {
            val: res,
            size: self.size,
        }
    }
}

// component wise
impl<T> std::ops::Add<Matrix<T>> for Matrix<T>
where
    T: std::ops::Add<Output = T> + std::fmt::Debug + std::default::Default + Copy + Clone,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.size != rhs.size {
            panic!("different size {:?} {:?}", self, rhs);
        } else {
            let mut res = vec![vec![T::default(); rhs.size.1]; self.size.0];
            for i in 0..self.size.0 {
                for j in 0..self.size.1 {
                    res[i][j] = self.val[i][j] + rhs.val[i][j];
                }
            }
            Matrix {
                val: res,
                size: self.size,
            }
        }
    }
}

#[test]
fn check_ops() {
    println!("{:?}", Matrix::<i64>::i(5) + Matrix::<i64>::i(5));
    println!("{:?}", Matrix::<i64>::i(5) - Matrix::<i64>::i(5));
    println!("{:?}", Matrix::<i64>::i(5) * Matrix::<i64>::i(5));
    let m1 = Matrix::from(vec![vec![3, 2, 1], vec![1, 4, 5], vec![1, 2, 3]]);
    let m2 = Matrix::from(vec![vec![1, 2, 3], vec![3, 2, 1], vec![1, 3, 2]]);
}

// component wise
impl<T> std::ops::Sub<Matrix<T>> for Matrix<T>
where
    T: std::ops::Sub<Output = T> + std::fmt::Debug + std::default::Default + Copy + Clone,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.size != rhs.size {
            panic!("different size {:?} {:?}", self, rhs);
        } else {
            let mut res = vec![vec![T::default(); rhs.size.1]; self.size.0];
            for i in 0..self.size.0 {
                for j in 0..self.size.1 {
                    res[i][j] = self.val[i][j] - rhs.val[i][j];
                }
            }
            Matrix {
                val: res,
                size: self.size,
            }
        }
    }
}

// dot product
// i*j x j * k
impl<T> std::ops::Mul<Matrix<T>> for Matrix<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::fmt::Debug
        + std::default::Default
        + Copy
        + Clone,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.size != rhs.size {
            panic!("different size {:?} {:?}", self, rhs);
        } else {
            let mut res = vec![vec![T::default(); rhs.size.1]; self.size.0];
            for i in 0..self.size.0 {
                for j in 0..self.size.1 {
                    for k in 0..rhs.size.1 {
                        res[i][k] = res[i][k] + (self.val[i][j] * rhs.val[j][k]);
                    }
                }
            }
            Matrix {
                val: res,
                size: self.size,
            }
        }
    }
}

#[test]
fn check_gen() {
    println!("{:?}", Matrix::<i64>::i(5));
    let m = vec![vec![1, 2, 34], vec![1, 2, 34], vec![1, 2, 34]];
    println!("{:?}", Matrix::from(m));
}

fn main() {}
