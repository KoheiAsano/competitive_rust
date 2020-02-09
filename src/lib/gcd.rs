// ほんとはGenericsにしたいけど一旦u64
/*
fn gcd<T: PartialOrd + std::ops::Rem>(a: T, b: T) -> T {
    if b == 0 {
        return a;
    } else if a < b {
        return gcd(a, b % a);
    } else {
        return gcd(b, a % b);
    }
}
*/



fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    } else if a < b {
        return gcd(a, b % a);
    } else {
        return gcd(b, a % b);
    }
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn symmetry() {
        assert_eq!(gcd(4, 6), gcd(6, 4));
        assert_eq!(gcd(17, 97), gcd(97, 17));
        assert_eq!(gcd(1000000007, 998244353), gcd(998244353, 1000000007));
    }
    #[test]
    fn one() {
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(17, 998244353), 1);
        assert_eq!(gcd(1000000007, 998244353), 1);
        for i in 1..100000 {
            assert_eq!(gcd(i, 998244353), 1);
        }
    }
    #[test]
    fn not_one() {
        assert_eq!(gcd(561, 17), 17);
        assert_eq!(gcd(255, 17), 17);
    }
}
