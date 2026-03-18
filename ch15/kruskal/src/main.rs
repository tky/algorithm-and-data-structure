mod union_find;

// 無向グラフ from, to, weight
type Edge = (usize, usize, i64);

// クラスカル法
// 無向グラフの辺のリストと頂点数を受け取って、最小全域木の重みを返す
// 連結グラフが前提
fn kruskal(eges: &mut [Edge], n: usize) -> i64 {
    let mut uf = union_find::UnionFind::new(n);
    let mut total_weight = 0;

    eges.sort_unstable_by_key(|e| e.2);

    for &(from, to, weight) in eges.iter() {
        if uf.union(from, to) {
            total_weight += weight;
        }
    }
    total_weight
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kruskal_triangle() {
        // 0-1 (1), 1-2 (2), 0-2 (3)
        // MST = 1 + 2 = 3
        let mut edges = vec![(0, 1, 1), (1, 2, 2), (0, 2, 3)];
        assert_eq!(kruskal(&mut edges, 3), 3);
    }

    #[test]
    fn test_kruskal_square_with_diagonal() {
        // 0---1
        // | X |
        // 3---2
        //
        // 辺:
        // 0-1:1
        // 1-2:2
        // 2-3:1
        // 3-0:2
        // 0-2:10
        //
        // MST = 1 + 1 + 2 = 4
        let mut edges = vec![(0, 1, 1), (1, 2, 2), (2, 3, 1), (3, 0, 2), (0, 2, 10)];
        assert_eq!(kruskal(&mut edges, 4), 4);
    }

    #[test]
    fn test_kruskal_already_tree() {
        // もともと木
        // MST = 全辺の和 = 5 + 7 + 2 = 14
        let mut edges = vec![(0, 1, 5), (1, 2, 7), (2, 3, 2)];
        assert_eq!(kruskal(&mut edges, 4), 14);
    }

    #[test]
    fn test_kruskal_unsorted_edges() {
        // 入力順がバラバラでも正しくソートして求まることを確認
        // MST = 1 + 2 + 3 = 6
        let mut edges = vec![(0, 3, 10), (1, 2, 2), (0, 1, 1), (2, 3, 3), (0, 2, 100)];
        assert_eq!(kruskal(&mut edges, 4), 6);
    }

    #[test]
    fn test_kruskal_with_negative_weights() {
        // 負辺があってもクラスカル法自体はそのまま使える
        // MST = -2 + 1 + 3 = 2
        let mut edges = vec![(0, 1, -2), (1, 2, 1), (2, 3, 3), (0, 3, 10), (0, 2, 4)];
        assert_eq!(kruskal(&mut edges, 4), 2);
    }

    #[test]
    fn test_kruskal_ignores_cycle_edge() {
        // 軽い3辺で木が完成した後、重い閉路辺は無視される
        // MST = 1 + 1 + 1 = 3
        let mut edges = vec![(0, 1, 1), (1, 2, 1), (2, 3, 1), (0, 3, 100), (0, 2, 50)];
        assert_eq!(kruskal(&mut edges, 4), 3);
    }
}
