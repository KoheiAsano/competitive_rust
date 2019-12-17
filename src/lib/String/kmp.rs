
fn kmp(pattern: &Vec<char>, target: &Vec<char>) -> Option<Vec<usize>> {
    let pl = pattern.len();
    let tl = target.len();
    let zurashi = {
        let mut tmp = 0;
        let mut res = vec![0];
        for i in 1..pl {
            if pattern[i] == pattern[tmp] {
                res.push(tmp);
                tmp += 1;
            } else {
                res.push(tmp);
                tmp = 0;
            }
        }
        res
    };
    let mut p = 0;
    let mut i = 0;
    let mut res = vec![];
    while i < tl && p < pl {
        if target[i] == pattern[p] {
            p += 1;
            i += 1;
        } else if p == 0 {
            i += 1;
        } else {
            p = zurashi[p];
        }
        if p == pl {
            res.push(i - p);
            p = if target[i - 1] == pattern[0] { 1 } else { 0 };
        }
    }
    if res.len() > 0 {
        Some(res)
    } else {
        None
    }
}
mod tests {
    use super::*;
    #[test]
    fn check_kmp() {
        let s = String::from("aababbababcab");
        let t = String::from("ababca");

        assert_eq!(
            kmp(&t.chars().collect(), &s.chars().collect()),
            Some(vec![6])
        );

        let s = String::from("aababbababcab");
        let t = String::from("ab");

        assert_eq!(
            kmp(&t.chars().collect(), &s.chars().collect()),
            Some(vec![1, 3, 6, 8, 11])
        );
    }

}
