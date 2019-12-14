fn accumulate_sum(v: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![0; v.len() + 1];
    for i in 1..res.len() {
        res[i] += v[i - 1] + res[i - 1];
    }
    res
}