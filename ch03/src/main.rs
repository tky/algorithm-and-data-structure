fn main() {
    println!("Hello, world!");
}

// 3.1
fn find_max(ns: &[u32], v: u32) -> usize {
    let mut found_id = 0;
    for i in 0..ns.len() {
        if ns[i] == v {
            found_id = i;
        }
    }
    found_id
}

// 3.2
fn count(ns: &[u32], v: u32) -> u32 {
    let mut count = 0;
    for &n in ns {
        if n == v {
            count += 1;
        }
    }
    count
}

// 3.3
fn find_next_minimum(ns: &[u32]) -> u32 {
    let mut min = u32::MAX;
    let mut next_min = u32::MAX;
    for &n in ns {
        if min > n {
            next_min = min;
            min = n;
        } else if next_min > n && n > min {
            next_min = n;
        }
    }
    next_min
}

// 3.4
fn find_max_difference(ns: &[u32]) -> u32 {
    let mut max = u32::MIN;
    let mut min = u32::MAX;

    for &n in ns {
        if max < n {
            max = n;
        }
        if min > n {
            min = n;
        }
    }
    max - min
}

// 3.5
fn count_operation(ns: &[u32]) -> u32 {
    // 2で割れる回数は2進数での末尾の0の個数に等しい
    // この値の最小値を探せば良い
    // 実際はmapしてminをとるが、計算量をわかりやすくkするためにループで実装
    let mut ans = u32::MAX;
    for &n in ns {
        let count = n.trailing_zeros();
        print!("n: {}, count: {}\n", n, count);
        if count != 0 && ans > count {
            ans = count;
        }
    }
    ans
}

// 3.6
// ２つの正の整数K, Nが与えられている
// 0 <= X, Y, Z <= Kを満たす整数X, Y, Zの組であって、
// X + Y + Z = Nを満たすものの個数を求める
fn count_combinations(k: u32, n: u32) -> u32 {
    // X + Y + Z = Nより
    // X + Y = N - Z
    // Z <= Kよりなので、右辺は N - K 以上 N以下の値をとる
    // X + Yが特定の数値Mになる組み合わせの数は M + 1通り
    // たとえば M = 2 のとき (0,2), (1,1), (2,0) の3通り

    if k < n {
        return 0;
    }

    let mut count = 0;
    for z in 0..=k {
        let m = n - z;
        count += m + 1;
    }
    count
}

// 3.7
// 参考サイト
// https://drken1215.hatenablog.com/entry/2019/12/14/171657
// https://drken1215.hatenablog.com/entry/2020/04/11/171900
fn sum_of_combinations(s: &str) -> u32 {
    let len = s.len();
    let mut sum = 0;
    for bit in 0..(1 << (len - 1)) {
        let mut tmp = 0;
        for n in 0..len {
            tmp *= 10;
            tmp += s.chars().nth(n).unwrap().to_digit(10).unwrap();
            if (bit & (1 << n)) != 0 {
                sum += tmp;
                tmp = 0;
            }
        }
        sum += tmp;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[1, 2, 3, 3, 2], 3), 3);
    }

    #[test]
    fn test_count() {
        assert_eq!(count(&[1, 2, 3, 3, 2], 2), 2);
    }

    #[test]
    fn test_find_next_minimum() {
        assert_eq!(find_next_minimum(&[5, 1, 4, 2, 1, 3]), 2);
    }

    #[test]
    fn test_find_max_difference() {
        assert_eq!(find_max_difference(&[1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_count_operation() {
        assert_eq!(count_operation(&[8, 4, 2, 16]), 1);
        assert_eq!(count_operation(&[8, 12, 40]), 2);
    }

    #[test]
    fn test_count_combinations() {
        assert_eq!(count_combinations(3, 3), 10);
        assert_eq!(count_combinations(2, 4), 0);
    }

    #[test]
    fn test_sum_of_combinations() {
        assert_eq!(sum_of_combinations("125"), 176);
    }
}
