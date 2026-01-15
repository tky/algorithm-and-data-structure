use superslice::*;

fn main() {
    println!("Hello, world!");
}

// 長さnの配列a,bが与えられる。
// a[i] * b[j]を昇順に並べた時にk番目に小さい値を求める
fn resolve(n: usize, aa: &mut [usize], bb: &mut [usize], k: usize) -> usize {
    // 前処理 a, bをソート
    // O(n log n)
    aa.sort_unstable();
    bb.sort_unstable();

    // a[i]を固定して考えた時にある値X以下となるa[i] * b[j]の積が何個何個あるか考える
    // a[i] * b[j] <= X => b[j] <= floor(X / a[i])
    let mut left = 0;
    let mut right = 1_000_000_000_000_000_000;

    while left < right {
        let mid = (left + right) / 2;
        let mut count = 0;
        for &a in aa.iter() {
            count += bb.upper_bound(&(mid / a));
        }
        // countが足りないと言うことは現在のmid以下の値はすべてNG
        // なのでmidの次の値をleftにして再検索する
        if count < k {
            left = mid + 1;
        // midも条件を満たすが、さらに小さい値が条件を満たす可能性がある
        // なのでmidを含めた値をrightにして再検索する
        } else {
            right = mid;
        }
    }
    // 最終的にleft == rightとなる
    // left未満の値はすべてNG (count(x) < k)
    // right以上の値はすべてOK (count(x) >= k)
    // よってrightが答えとなっている
    right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve1() {
        let n = 2;
        let mut aa = vec![2, 3];
        let mut bb = vec![3, 5];
        let k = 3;
        let result = resolve(n, &mut aa, &mut bb, k);
        assert_eq!(result, 10);
    }
    #[test]
    fn test_resolve2() {
        let n = 3;
        let mut aa = vec![1, 2, 1];
        let mut bb = vec![2, 1, 2];
        let k = 7;
        let result = resolve(n, &mut aa, &mut bb, k);
        assert_eq!(result, 2);
    }
}
