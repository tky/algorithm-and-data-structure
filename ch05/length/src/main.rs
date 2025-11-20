fn main() {
    let s = "kitten";
    let t = "sitting";
    let result = length(s, t);
    println!(
        "The edit distance between '{}' and '{}' is {}",
        s, t, result
    );
}

fn chmin<T: PartialOrd + Copy>(a: &mut T, b: T) {
    if *a > b {
        *a = b;
    }
}

fn length(s: &str, t: &str) -> usize {
    let ss = s.chars().collect::<Vec<char>>();
    let ts = t.chars().collect::<Vec<char>>();

    let mut dp = vec![vec![usize::MAX; ts.len() + 1]; ss.len() + 1];
    dp[0][0] = 0;

    for i in 0..=ss.len() {
        for j in 0..=ts.len() {
            if i > 0 && j > 0 {
                // 変更操作
                // s[i - 1] == t[j - 1]の時: コストを増やさずに住む
                if ss[i - 1] == ts[j - 1] {
                    let cost = dp[i - 1][j - 1];
                    chmin(&mut dp[i][j], cost);
                } else {
                    let cost = dp[i - 1][j - 1] + 1;
                    chmin(&mut dp[i][j], cost);
                }
            }

            // 削除操作
            // sのi文字目を削除
            if i > 0 {
                let cost = dp[i - 1][j] + 1;
                chmin(&mut dp[i][j], cost);
            }
            // 挿入操作
            // tのj文字目を削除
            if j > 0 {
                let cost = dp[i][j - 1] + 1;
                chmin(&mut dp[i][j], cost);
            }
        }
    }
    dp[ss.len()][ts.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length() {
        assert_eq!(length("logistic", "algorithm"), 6);
        assert_eq!(length("kitten", "sitting"), 3);
        assert_eq!(length("flaw", "lawn"), 2);
        assert_eq!(length("intention", "execution"), 5);
    }
}
