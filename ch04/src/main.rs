use std::collections::HashMap;

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
}
