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
}
