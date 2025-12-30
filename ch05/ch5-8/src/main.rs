use core::f64;

fn main() {
    println!("Hello, world!");
}

fn average(vs: &[usize], l: usize, r: usize) -> f64 {
    (vs[l..r].iter().sum::<usize>() as f64) / ((r - l) as f64)
}

fn resolve(vs: &[usize], m: usize) -> f64 {
    // dp[k][i] := 先頭からi個をk区間に分割した時の平均の総総の最大値
    // dp[k][i] = max(dp[k-1][j] + average(j, i)
    let mut dp = vec![vec![f64::NEG_INFINITY; vs.len() + 1]; m + 1];
    dp[0][0] = 0.0;
    for k in 1..=m {
        for i in 0..=vs.len() {
            for j in k - 1..i {
                dp[k][i] = dp[k][i].max(dp[k - 1][j] + average(vs, j, i));
            }
        }
    }
    dp[m][vs.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve() {
        let vs = vec![9, 1, 2, 3, 9];
        let m = 3;
        let result = resolve(&vs, m);
        assert_eq!(result, 20.0);
    }
}
