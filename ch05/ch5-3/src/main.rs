fn main() {
    println!("Hello, world!");
}

// N個の整数と正の整数Wが与えられる。
// いくつか選んで総和をとって得られる1以上W以下の整数が何通りあるか
fn resolve(vs: &Vec<usize>, w: usize) -> usize {
    let len = vs.len();
    // i番目までを使って総和jが作れるか
    let mut dp = vec![vec![false; w + 1]; len + 1];
    for i in 0..len {
        for j in 0..=w {
            // 選んだ数が目標の総和の場合
            if vs[i] == j {
                dp[i + 1][j] = true;
            }
            // i番目を使わなくてもすでに総和が作れている場合
            if dp[i][j] {
                dp[i + 1][j] = true;
            }
            // i番目を使って総和ができる場合 => i - 1番目まででj - vs[i]が作れている場合
            if j > vs[i] && dp[i][j - vs[i]] {
                dp[i + 1][j] = true;
            }
        }
    }
    let mut ans = 0;
    for j in 1..=w {
        if dp[len][j] {
            ans += 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve() {
        let vs = vec![1, 3, 5];
        let w = 7;
        assert_eq!(resolve(&vs, w), 5);
    }

    #[test]
    fn test_all_same() {
        let vs = vec![2, 2, 2];
        let w = 6;
        // 2, 4, 6 の3通り
        assert_eq!(resolve(&vs, w), 3);
    }

    #[test]
    fn test_single_element() {
        let vs = vec![4];
        let w = 5;
        // 4のみ
        assert_eq!(resolve(&vs, w), 1);
    }

    #[test]
    fn test_large_element() {
        let vs = vec![10, 1, 2];
        let w = 5;
        // 1,2,3(1+2)
        assert_eq!(resolve(&vs, w), 3);
    }

    #[test]
    fn test_empty() {
        let vs = vec![];
        let w = 10;
        // 何も選べないので0
        assert_eq!(resolve(&vs, w), 0);
    }
}
