fn main() {
    let a = vec![3, 2, 5, 10, 7];
    println!("Hello, world!");
}

fn partial_sum(vs: &Vec<usize>, w: usize) -> bool {
    let len = vs.len();
    // i番目までを使って和がjになるか
    let mut dp = vec![vec![false; w + 1]; len + 1];

    // 初期値
    // 0は作れる
    dp[0][0] = true;

    for i in 0..len {
        for j in 0..=w {
            if dp[i][j] {
                dp[i + 1][j] = true;
            } else if j >= vs[i] {
                dp[i + 1][j] = dp[i][j - vs[i]];
            }
        }
    }
    dp[len][w]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_partial_sum() {
        let v1 = vec![3, 2, 5, 10, 7];
        assert_eq!(partial_sum(&v1, 15), true);
        assert_eq!(partial_sum(&v1, 14), true);
        assert_eq!(partial_sum(&v1, 11), false);
        assert_eq!(partial_sum(&v1, 18), true);
    }
}
