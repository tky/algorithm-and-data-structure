use std::vec;

fn main() {
    let a = vec![3, 7, 6];
    let b = vec![2, 5, 4];
    let c = vec![1, 8, 9];
    let n = a.len();
    let result = calculate_happiness(&a, &b, &c, n);
    println!("最大幸福値: {}", result);
}

fn calculate_happiness(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>, n: usize) -> i32 {
    const A: usize = 0;
    const B: usize = 1;
    const C: usize = 2;

    // dp table
    // i日目にj(A, B, C)を選んだ時の最大幸福値
    let mut dp = vec![vec![0; 3]; n];

    // 初日の選択肢を初期化
    dp[0] = vec![a[0], b[0], c[0]];

    for i in 1..n {
        dp[i][A] = (dp[i - 1][B] + a[i]).max(dp[i - 1][C] + a[i]);
        dp[i][B] = (dp[i - 1][A] + b[i]).max(dp[i - 1][C] + b[i]);
        dp[i][C] = (dp[i - 1][A] + c[i]).max(dp[i - 1][B] + c[i]);
    }
    dp[n - 1].iter().copied().max().unwrap()
}

// 改良版
// i日目の状態はi-1日目の状態のみ分かれば良いので前日分だけ保持するように修正
fn calculate_happiness2(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>, n: usize) -> i32 {
    const A: usize = 0;
    const B: usize = 1;
    const C: usize = 2;

    // dp table
    let mut dp = vec![a[0], b[0], c[0]];

    for i in 1..n {
        let a = (dp[B] + a[i]).max(dp[C] + a[i]);
        let b = (dp[A] + b[i]).max(dp[C] + b[i]);
        let c = (dp[A] + c[i]).max(dp[B] + c[i]);
        dp = vec![a, b, c];
    }
    dp.iter().copied().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_happiness() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = vec![7, 8, 9];
        let n = a.len();
        let result = calculate_happiness(&a, &b, &c, n);
        assert_eq!(result, 21);
        assert_eq!(calculate_happiness2(&a, &b, &c, n), 21);
    }

    #[test]
    fn test_calculate_happiness2() {
        let a = vec![6, 8, 2, 7, 4, 2, 7];
        let b = vec![7, 8, 5, 8, 6, 3, 5];
        let c = vec![8, 3, 2, 6, 8, 4, 1];
        let result = calculate_happiness(&a, &b, &c, 7);
        assert_eq!(result, 46);
        assert_eq!(calculate_happiness2(&a, &b, &c, 7), 46);
    }
}
