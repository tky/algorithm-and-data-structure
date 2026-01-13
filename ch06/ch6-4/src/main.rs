// ch6-4
// Aggressive cows
fn main() {
    println!("Hello, world!");
}

// vsからm個の要素を選んで、最小の距離を返す
fn resolve(vs: &mut [usize], m: usize) -> usize {
    vs.sort();
    let mut left = 0;
    // ２分探索において通常
    // left: 可能
    // right: 不可能
    // とするのが定石なのでrightは不可能な値の最小値である最大値 + 1に設定する
    let mut right = vs[vs.len() - 1] - vs[0] + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if can_place(vs, m, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

// vsからm個の要素を選んで、distance以上の距離を保てるかどうか
fn can_place(vs: &[usize], m: usize, distance: usize) -> bool {
    let mut count = 1usize;
    // 最初は一番左におく
    let mut last = vs[0];

    for &pos in vs.iter().skip(1) {
        // 前回置いた場所からdistance以上離れているなら置ける
        if pos - last >= distance {
            count += 1;
            last = pos;
            if count >= m {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve() {
        let vs = vec![1, 2, 8, 4, 9];
        let m = 3;
        assert_eq!(resolve(&mut vs.clone(), m), 3);
    }

    #[test]
    fn test_can_place() {
        let vs = vec![1, 2, 4, 8, 9];
        let m = 3;
        assert_eq!(can_place(&vs, m, 2), true);
        assert_eq!(can_place(&vs, m, 3), true);
        assert_eq!(can_place(&vs, m, 4), false);
    }
}
