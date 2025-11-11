use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

// 4.1 trionacci
fn tribonacci(n: usize) -> u64 {
    match n {
        0 | 1 => 0,
        2 => 1,
        _ => tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3),
    }
}

// 4.2 tribonacci with memoization
fn tribonacci_memo(n: usize) -> u64 {
    let mut memo: HashMap<usize, u64> = HashMap::new();

    fn _go(n: usize, memo: &mut HashMap<usize, u64>) -> u64 {
        match memo.get(&n) {
            Some(&value) => value,
            None => match n {
                0 | 1 => {
                    memo.insert(n, 0);
                    0
                }
                2 => {
                    memo.insert(2, 1);
                    1
                }
                _ => {
                    let v = _go(n - 1, memo) + _go(n - 2, memo) + _go(n - 3, memo);
                    memo.insert(n, v);
                    v
                }
            },
        }
    }
    _go(n, &mut memo)
}

fn find_under_753(k: u64) -> usize {
    let d = digits(k);
    let mut count = 0;
    for i in 0..d {
        for n in generate(d - i) {
            if n <= k && is_753_number(n) {
                count += 1;
            }
        }
    }
    count
}

fn generate(d: u32) -> Vec<u64> {
    if d == 1 {
        return vec![7, 5, 3];
    }
    let base = 10u64.pow((d - 1) as u32);
    let mut result = Vec::new();
    for &n in &[7 * base, 5 * base, 3 * base] {
        for m in generate(d - 1) {
            result.push(n + m);
        }
    }
    result
}

fn is_753_number(n: u64) -> bool {
    let uniq: HashSet<char> = n.to_string().chars().collect();
    uniq.len() == 3
}

#[inline]
fn digits(n: u64) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

// 部分和問題を再帰関数を使って全捜索する
// vsのなかから最初のi個を使ってwが作れるかどうか
fn naive_partial_sum(i: usize, w: usize, vs: &[usize]) -> bool {
    if w == 0 {
        return true;
    }
    if i == 0 {
        return false;
    }

    // vs[i-1]を使わない場合
    if naive_partial_sum(i - 1, w, vs) {
        return true;
    }

    // vs[i-1]を使う場合
    if w >= vs[i - 1] {
        if naive_partial_sum(i - 1, w.saturating_sub(vs[i - 1]), vs) {
            return true;
        }
    }
    false
}

fn partial_sum_memo(i: usize, w: usize, vs: &[usize]) -> bool {
    let mut memo: HashMap<(usize, usize), bool> = HashMap::new();

    fn _go(i: usize, w: usize, vs: &[usize], memo: &mut HashMap<(usize, usize), bool>) -> bool {
        match memo.get(&(i, w)) {
            Some(&value) => value,
            None => {
                if w == 0 {
                    return true;
                }
                if i == 0 {
                    return false;
                }
                let a = vs[i - 1];
                let ans = _go(i - 1, w, vs, memo) || (w >= a && _go(i - 1, w - a, vs, memo));
                memo.insert((i, w), ans);
                ans
            }
        }
    }

    _go(i, w, vs, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tribonacci() {
        assert_eq!(tribonacci(0), 0);
        assert_eq!(tribonacci(1), 0);
        assert_eq!(tribonacci(2), 1);
        assert_eq!(tribonacci(3), 1);
        assert_eq!(tribonacci(4), 2);
        assert_eq!(tribonacci(5), 4);
        assert_eq!(tribonacci(6), 7);
        assert_eq!(tribonacci(7), 13);
    }

    #[test]
    fn test_tribonacci_memo() {
        assert_eq!(tribonacci_memo(0), 0);
        assert_eq!(tribonacci_memo(1), 0);
        assert_eq!(tribonacci_memo(2), 1);
        assert_eq!(tribonacci_memo(3), 1);
        assert_eq!(tribonacci_memo(4), 2);
        assert_eq!(tribonacci_memo(5), 4);
        assert_eq!(tribonacci_memo(6), 7);
        assert_eq!(tribonacci_memo(7), 13);
    }

    #[test]
    fn test_find_under_753() {
        assert_eq!(find_under_753(400), 2);
        assert_eq!(find_under_753(575), 4);
    }

    #[test]
    fn test_naive_partial_sum() {
        let vs = vec![3, 34, 4, 12, 5, 2];
        assert_eq!(naive_partial_sum(vs.len(), 9, &vs), true);
        assert_eq!(naive_partial_sum(vs.len(), 1, &vs), false);
    }

    #[test]
    fn test_partial_sum_memo() {
        let vs = vec![3, 34, 4, 12, 5, 2];
        assert_eq!(partial_sum_memo(vs.len(), 9, &vs), true);
        assert_eq!(partial_sum_memo(vs.len(), 1, &vs), false);
    }
}
