fn main() {
    println!("Hello, world!");
}

fn chmax<T: PartialOrd + Copy>(a: &mut T, b: T) {
    if *a < b {
        *a = b;
    }
}

fn lcs_length(s: &str, t: &str) -> String {
    let ss = s.chars().collect::<Vec<char>>();
    let ts = t.chars().collect::<Vec<char>>();

    // dp[i][j] := s[0..i]とt[0..j]の最長共通部分列の長さ
    let mut dp = vec![vec![0; ts.len() + 1]; ss.len() + 1];

    for i in 1..=ss.len() {
        for j in 1..=ts.len() {
            // s[i-1]とt[j-1]の比較に関係なく、どちらか片方を捨てた時の最長共通部分列の長さを引き継ぐ
            // s[i-1]を使わない場合 => dp[i-1][j]
            // t[j-1]を使わない場合 => dp[i][j-1]
            // まずはその大きい方を採用する
            dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            // その上で、s[i-1]とt[j-1]が等しければ長長さ伸ばせる
            if ss[i - 1] == ts[j - 1] {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
            }
        }
    }

    // 復元
    let mut i = ss.len();
    let mut j = ts.len();
    let mut out: Vec<char> = Vec::new();

    while i > 0 && j > 0 {
        if ss[i - 1] == ts[j - 1] {
            out.push(ss[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    out.reverse();
    out.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lcs_length() {
        assert_eq!(lcs_length("axyb", "abyxb"), "ayb");
        assert_eq!(lcs_length("aa", "xayaz"), "aa");
    }
}
