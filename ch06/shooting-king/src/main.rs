fn main() {
    println!("Hello, world!");
}

// nこの風船が初期状態で高度h[i]の位置にあり、1秒ごとにs[i]だけ上昇する
// 1秒ごとに1この風船を割ることができる
// 風船を割る時のペナルティはその時の風船の高度
// すべての風船を割るときのペナルティの最小値を求めよ

// すべての風船をx以下の高度で割れるか２分捜索で求よ、という問題に帰着できる
fn resolve(n: usize, h: &[usize], s: &[usize]) -> usize {
    let mut left = 0;
    // 適当に十分な大きさの値
    let mut right = 10000000;

    while right - left > 1 {
        let mid = (left + right) / 2;

        let mut ok = true;
        let mut times = vec![0; n];
        for i in 0..n {
            // 初期値よりも低い場合割ることができない
            if mid < h[i] {
                ok = false;
                break;
            }
            // iが高度X以下であるためには
            // Hi + Si * t <= X
            // より
            // t <= (X - Hi) / Si
            // tまでに割らなければ高度Xを超えてしまう
            times[i] = (mid - h[i]) / s[i];
        }

        if ok {
            // 直感的に、締切が早い風船を先に割る必要があるためソートする
            times.sort();
            for i in 0..n {
                // i秒目までに破れない場合は失敗
                if times[i] < i {
                    ok = false;
                    break;
                }
            }
        }

        if ok {
            right = mid;
        } else {
            left = mid;
        }
    }
    right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve() {
        let n = 4;
        let h = vec![5, 12, 14, 21];
        let s = vec![6, 4, 7, 2];
        assert_eq!(resolve(n, &h, &s), 23);
    }
}
