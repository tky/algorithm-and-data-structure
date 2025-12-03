fn main() {
    println!("Hello, world!");
}

// N個の整数と正の整数Wが与えられる。
// いくつか選んで総和をwにできるかできるか？
// ただし使える数はk個までとする
fn resolve(vs: &Vec<usize>, w: usize, k: usize) -> bool {
    let len = vs.len();
    // 前回は値が作れるかどうかだけだったのでbooleanで良かったが
    // 今回は幾つの数で作れるかを持つようにする
    // 目標の数が作れる最小値を知りたいので最大値で初期化
    let mut dp = vec![vec![usize::MAX; w + 1]; len + 1];

    dp[0][0] = 0; // 0個で0は作れる
    for i in 0..len {
        for j in 0..=w {
            // i番目を使わない場合
            dp[i + 1][j] = dp[i][j];
            // i番目を使う場合
            if j >= vs[i] && dp[i][j - vs[i]] != usize::MAX {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j - vs[i]] + 1);
            }
        }
    }
    dp[len][w] <= k
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_resolve() {
        let vs = vec![2, 3, 5, 7];
        assert_eq!(resolve(&vs, 10, 3), true); // 3 + 5 + 2
        assert_eq!(resolve(&vs, 10, 2), true); // 7 + 3
        assert_eq!(resolve(&vs, 14, 2), false);
        assert_eq!(resolve(&vs, 1, 1), false);
        assert_eq!(resolve(&vs, 4, 1), false);
        assert_eq!(resolve(&vs, 4, 2), false);
    }
}
