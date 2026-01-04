fn main() {
    println!("Hello, world!");
}

fn resolve(vs: &[usize]) -> usize {
    let n = vs.len();
    if n <= 1 {
        return 0;
    }

    // 累積和の計算
    // 任意のvs[l..r] の和をO(1)で求めるために事前に計算しておく
    // 例
    // [1..3)の和はsum[3] - sum[1]で求まる
    let mut ps = vec![0usize; n + 1];
    for i in 0..n {
        ps[i + 1] = ps[i] + vs[i];
    }
    let sum = |l: usize, r: usize, ps: &Vec<usize>| -> usize { ps[r] - ps[l] };

    // 加算で溢れないように
    let inf = usize::MAX / 4;
    // dp[l][r]: 区間[l, r)を一つにまとめた時の最小コスト
    let mut dp = vec![vec![inf; n + 1]; n + 1];

    // 長さ1（区間 [i, i+1)）はコスト0
    for i in 0..n {
        dp[i][i + 1] = 0;
    }

    // 区間長 len = 2..=n の順に計算（短い区間 → 長い区間）
    for len in 2..=n {
        for l in 0..=n - len {
            let r = l + len;
            // 区間[l, r)の和
            let s = sum(l, r, &ps);
            let mut best = inf;

            for k in (l + 1)..r {
                // [l, k)と[k, r)に分割した場合のコストを計算
                best = best.min(dp[l][k] + dp[k][r] + s);
            }
            dp[l][r] = best;
        }
    }

    dp[0][n]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve() {
        assert_eq!(resolve(&vec![1, 2, 3, 4, 5]), 33);
    }
}
