
fn abs(a: i32) -> i32 {
    let b = a >> 31;
    return (b ^ a) - b;
}

fn min(a: i64, b: i64) -> i64 {
    return b ^ ((a ^ b) & -((a < b) as i64));
}

fn main() {
    println!("{:?}", abs(9));
    println!("{:?}", min(1, 2));
    println!("{:?}", min(2, 1));

    println!("{:?}", min(2, -1));
    println!("{:?}", min(-1, 2));

    println!("{:?}", min(-2, -1));
    println!("{:?}", min(-1, -2));

    println!("{:?}", min(-2, 0));
    println!("{:?}", min(0, -2));

    println!("{:?}", min(0, 0));

    println!("{:?}", min(1, 0));
    println!("{:?}", min(0, 1));
}
