use std::collections::{BTreeMap, BTreeSet};

use superslice::*;
/**
 * 二次元平面上に、赤い点と青い点が N
 * 個ずつあります。i個目の赤い点の座標は (ai,bi)
 * であり、i個目の青い点の座標は (ci,di)
 * です。
 * 赤い点と青い点は、x座標と y座標がともに赤い点よりも青い点の方が大きいとき、仲良しペアになれます。
 * ただし，1 つの点が複数のペアに所属することはできません。
 * あなたは最大で何個の仲良しペアを作ることができますか？
 */

fn main() {
    println!("Hello, world!");
}

type Point = (i32, i32);

// 赤い点のx座標のものと、その点とペアにできる青い点のx座標の最小のなかから、青い点のy座標が最大のものを選びペアにする
fn solve(red_points: &mut Vec<Point>, blue_points: &mut Vec<Point>) -> usize {
    red_points.sort_by_key(|&(x, _)| x);
    blue_points.sort_by_key(|&(x, _)| x);

    let mut count = 0;

    println!("Red Points (sorted by x): {:?}", red_points);
    println!("Blue Points (sorted by x): {:?}", blue_points);

    // まだ残っている赤い点のうち、
    // その青い点の左側(x座標が小さい側）にあって、
    // y座標が最大の点を選ぶ
    for &(bx, by) in blue_points.iter() {
        let idx = red_points.lower_bound_by(|&r| r.0.cmp(&bx));
        if idx == 0 {
            continue;
        }
        let best = red_points[..idx]
            .iter()
            .enumerate()
            .filter(|&(_offset, r)| r.1 < by)
            .max_by_key(|&(_offset, b)| b.1)
            .map(|(offset, &b)| (offset, b));

        if let Some((best_idx, (rx, ry))) = best {
            print!(
                "Blue Point: ({}, {}), Best Red: index={}, ({}, {})\n",
                bx, by, best_idx, rx, ry
            );
            red_points.remove(best_idx);
            count += 1;
        }
    }
    count
}

// 修正版
// vector.removeは重いので使わない方がいい
// rx < bx を満たす最大のy座標を探す処理はBTreeSetを使うと効率的にできる
fn solve2(mut red: Vec<Point>, mut blue: Vec<Point>) -> usize {
    red.sort_unstable_by_key(|p| p.0);
    blue.sort_unstable_by_key(|p| p.0);

    let mut ys: BTreeSet<i32> = BTreeSet::new();
    // まだ使っていない赤い点のうち、x座標が小さい順に取り出すためのインデックス
    let mut ri = 0;
    let mut ans = 0;

    for (bx, by) in blue {
        // 対象の青い点のx座標より小さい赤い点をすべてysに追加
        // riはまだysに入れていない赤点の先頭indexを保存している
        // なので２回同じ赤い点をysに入れることはない
        while ri < red.len() && red[ri].0 < bx {
            let y = red[ri].1;
            ys.insert(y);
            ri += 1;
        }

        // BTreeSetは順序付き集合（昇順にならんでいる）
        // range(..by)はby未満の要素だけを範囲として返す
        // .next_back()はその中で最大の値を返す
        if let Some(&y) = ys.range(..by).next_back() {
            ys.remove(&y);
            ans += 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut red_points = vec![(2, 0), (3, 1), (1, 3)];
        let mut blue_points = vec![(4, 2), (0, 4), (5, 5)];
        assert_eq!(solve2(red_points.to_vec(), blue_points.to_vec()), 2);
        assert_eq!(solve(&mut red_points, &mut blue_points), 2);
    }

    #[test]
    fn test_solve1() {
        let mut red_points = vec![(0, 0), (1, 1), (5, 5), (6, 6), (7, 7)];
        let mut blue_points = vec![(2, 2), (3, 3), (4, 4), (8, 8), (9, 9)];
        assert_eq!(solve2(red_points.to_vec(), blue_points.to_vec()), 4);
        assert_eq!(solve(&mut red_points, &mut blue_points), 4);
    }
}
