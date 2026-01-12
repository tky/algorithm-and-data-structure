use superslice::*;

fn main() {
    println!("Hello, world!");
}

// これだと計算量が厳しいようです。。
fn resolve(vs: &[usize], m: usize) -> usize {
    let ns = find(vs, m);

    let idx = ns.upper_bound(&m);
    ns[idx - 1]
}

// vsから4つの要素の和で作れる数値の集合を返す
fn find(vs: &[usize], target: usize) -> Vec<usize> {
    let mut dp = vec![5; target + 1];
    dp[0] = 0;
    for &v in vs {
        dp[v] = 1;
    }
    for i in 1..=target {
        for &v in vs {
            if i >= v {
                dp[i] = dp[i].min(dp[i - v] + 1);
            }
        }
    }
    let mut ns = Vec::<usize>::new();
    for (i, &b) in dp.iter().enumerate() {
        if b <= 4 {
            ns.push(i);
        }
    }
    ns
}

fn resolve2(vs: &[usize], m: usize) -> usize {
    let n = vs.len();
    let mut sums = Vec::with_capacity(n * (n + 1) / 2);

    // 1 or 2つの要素の和で作れる数値を列挙
    for i in 0..n {
        sums.push(vs[i]);
        for j in i..n {
            sums.push(vs[i] + vs[j]);
        }
    }
    sums.push(0);
    sums.sort_unstable();

    let mut ans = 0;

    // 1 or 2つの要素の和の組み合わせ、つまり1,2,3,4つの要素の和で作れる数値を調べる
    for &x in &sums {
        if x > m {
            break;
        }
        let rest = m - x;
        let idx = sums.upper_bound(&rest);
        if idx == 0 {
            continue;
        }
        let y = sums[idx - 1];
        ans = ans.max(x + y);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let vs = vec![3, 14, 15, 9];
        let ns = find(&vs, 50);
        assert_eq!(ns.contains(&48), true);
        assert_eq!(ns.contains(&49), false);
    }

    #[test]
    fn test_resolve() {
        let vs = vec![3, 14, 15, 9];
        assert_eq!(resolve(&vs, 50), 48);
        assert_eq!(resolve2(&vs, 50), 48);
    }

    #[test]
    fn test_resolve2() {
        let vs = vec![16, 11, 2];
        assert_eq!(resolve(&vs, 21), 20);
        assert_eq!(resolve2(&vs, 21), 20);
    }

    #[test]
    fn test_resolve3() {
        let vs = vec![16, 11, 9];
        assert_eq!(resolve(&vs, 17), 16);
        assert_eq!(resolve2(&vs, 17), 16);
    }
}
