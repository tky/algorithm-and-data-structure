mod union_find;

// 無向グラフ from, to, weight
type Edge = (usize, usize, i64);

// クラスカル法
// 連結グラフが前提
fn kruskal(edges: &mut [Edge], n: usize) -> i64 {
    let mut uf = union_find::UnionFind::new(n);
    let mut used = 0;

    edges.sort_unstable_by_key(|e| e.2);

    for &(from, to, weight) in edges.iter() {
        if uf.union(from, to) {
            used += 1;
            if used == n / 2 {
                return weight;
            }
        }
    }
    unreachable!("graph must be connected");
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n2_single_edge() {
        let mut edges = vec![(0, 1, 42)];
        assert_eq!(kruskal(&mut edges, 2), 42);
    }

    #[test]
    fn simple_unique_mst_n4() {
        // MST は (1, 2, 3) を採用
        // n = 4 なので 2本目の採用辺 = 2 が答え
        let mut edges = vec![(0, 1, 1), (1, 2, 2), (2, 3, 3), (0, 2, 10), (1, 3, 10)];
        assert_eq!(kruskal(&mut edges, 4), 2);
    }

    #[test]
    fn input_order_does_not_matter() {
        // 辺の入力順がバラバラでも sort されて正しく動くことを確認
        let mut edges = vec![(1, 3, 10), (2, 3, 3), (0, 2, 10), (1, 2, 2), (0, 1, 1)];
        assert_eq!(kruskal(&mut edges, 4), 2);
    }

    #[test]
    fn skips_cycle_edges() {
        // 0-1-2 の三角形があり、重み 3 の辺は cycle になるので採用されない
        // 採用順は 1, 2, 5, 6, 7
        // n = 6 なので 3本目の採用辺 = 5 が答え
        let mut edges = vec![
            (0, 1, 1),
            (1, 2, 2),
            (0, 2, 3), // cycle になるので不採用
            (2, 3, 5),
            (3, 4, 6),
            (4, 5, 7),
            (0, 5, 100),
        ];
        assert_eq!(kruskal(&mut edges, 6), 5);
    }

    #[test]
    fn equal_weight_edges() {
        // 同じ重みの辺があっても問題なく動くことを確認
        // MST の採用辺重みは 1, 1, 2
        // n = 4 なので答えは 1
        let mut edges = vec![(0, 1, 1), (1, 2, 1), (2, 3, 2), (0, 2, 2), (1, 3, 100)];
        assert_eq!(kruskal(&mut edges, 4), 1);
    }

    #[test]
    fn larger_case_n8() {
        // 採用順は 1, 2, 3, 4, 5, 6, 7
        // n = 8 なので 4本目の採用辺 = 4
        let mut edges = vec![
            (0, 1, 1),
            (1, 2, 2),
            (2, 3, 3),
            (3, 4, 4),
            (4, 5, 5),
            (5, 6, 6),
            (6, 7, 7),
            (0, 7, 100),
            (1, 6, 100),
            (2, 5, 100),
        ];
        assert_eq!(kruskal(&mut edges, 8), 4);
    }
}
