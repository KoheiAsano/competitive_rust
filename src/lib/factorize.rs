// ほんとはGenericにしたいけど一旦u64
/*
自分自身を含むけど1を含まない
*/

fn factorize(a: u64) -> Vec<u64> {
    let u: u64 = (a as f64).sqrt().ceil() as u64;
    let mut res: Vec<u64> = Vec::<u64>::new();
    for i in 2..u {
        if a % i == 0 {
            res.push(i);
            res.push(a / i);
        }
    }
    res.push(a);
    res.sort();
    res
}

#[cfg(test)]
mod tests {
    use super::factorize;

    #[test]
    fn carmichael() {
        assert_eq!(factorize(561), vec![3, 11, 17, 33, 51, 187, 561]);
    }
    #[test]
    fn prime() {
        assert_eq!(factorize(998244353), vec![998244353]);
    }
    #[test]
    fn perfect() {
        assert_eq!(factorize(496), vec![2, 4, 8, 16, 31, 62, 124, 248, 496])
    }
}

fn main() {}
