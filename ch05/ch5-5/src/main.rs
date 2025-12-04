fn main() {
    println!("Hello, world!");
}

fn resolve(vs: &Vec<usize>, w: usize) -> bool {
    // dp[i] := iが作れるかどうか
    let mut dp = vec![false; w + 1];
    // 初期値
    // 0は作れるとする
    dp[0] = true;
    for i in 1..=w {
        // vsの中のどれかを引いた値が作れるならiも作れる
        dp[i] = vs.iter().any(|&v| i >= v && dp[i - v]);
    }
    dp[w]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve() {
        let vs = vec![3, 5, 7];
        assert_eq!(resolve(&vs, 8), true);
        assert_eq!(resolve(&vs, 9), true);
        assert_eq!(resolve(&vs, 10), true);
        assert_eq!(resolve(&vs, 1), false);
        assert_eq!(resolve(&vs, 2), false);
        assert_eq!(resolve(&vs, 4), false);
        assert_eq!(resolve(&vs, 16), true);
        assert_eq!(resolve(&vs, 17), true);
    }

    #[test]
    fn test_resolve2() {
        let vs = vec![5, 11];
        assert_eq!(resolve(&vs, 9), false);
        assert_eq!(resolve(&vs, 21), true);
        assert_eq!(resolve(&vs, 22), true);
        assert_eq!(resolve(&vs, 23), false);
    }
}
