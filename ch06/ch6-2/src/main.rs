use superslice::*;
fn main() {
    println!("Hello, world!");
}

fn resolve(n: usize, a: &mut [usize], b: &mut [usize], c: &mut [usize]) -> usize {
    let mut result = 0;
    // sort3回
    // 3 * O(n log n) = O(n log n)
    a.sort();
    b.sort();
    c.sort();

    // b[j]を固定して考える
    for j in 0..n {
        // aの中でb[j]より小さいものの個数
        let i = a.lower_bound(&b[j]);
        // cの中でb[j]より大きいものの個数
        let k = c.upper_bound(&b[j]);
        result += i * (n - k);
    }
    // n * (O(log n) + O(log n)) = O(n log n)

    // 全体でO(n log n)
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve() {
        let n = 2;
        let mut a = vec![1, 5];
        let mut b = vec![2, 4];
        let mut c = vec![3, 6];
        assert_eq!(resolve(n, &mut a, &mut b, &mut c), 3);
    }

    #[test]
    fn test_resolve2() {
        let n = 3;
        let mut a = vec![1, 1, 1];
        let mut b = vec![2, 2, 2];
        let mut c = vec![3, 3, 3];
        assert_eq!(resolve(n, &mut a, &mut b, &mut c), 27);
    }
}
