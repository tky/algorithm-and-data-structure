// 有向グラフ（隣接リスト）
type Graph = Vec<Vec<usize>>;

// 有向辺 (from, to)
type Edge = (usize, usize);

// 辺集合
type Edges = Vec<Edge>;

fn build_graph(n: usize, edges: &Edges) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for &(from, to) in edges.iter() {
        g[from].push(to);
    }
    g
}

fn topological_sort(edges: &Edges, n: usize) -> Vec<usize> {
    // 入次数を数える
    let mut indegrees = vec![0; n];

    let mut results = Vec::new();

    for &(_from, to) in edges.iter() {
        indegrees[to] += 1;
    }

    while results.len() < n {
        // DAGの性質上、必ず入次数が0の頂点が存在する
        let mut v = 0;
        for i in 0..n {
            if indegrees[i] == 0 && !results.contains(&i) {
                v = i;
                break;
            }
        }
        results.push(v);
        // vから出る辺を削除する
        for &(from, to) in edges.iter() {
            if from == v {
                indegrees[to] -= 1;
            }
        }
    }
    results
}

fn find_longest_path(edges: &Edges, n: usize) -> usize {
    let order = topological_sort(edges, n);
    // dp[v]: 頂点vを終点とする最長パスの長さ
    let mut dp = vec![0; n];

    // トポロジカル順序で頂点を処理する
    // dp[to] = max(dp[to], dp[v] + 1)
    for &v in order.iter() {
        // 辺 v -> toがある場合、すでにdp[v]が計算されているので、dp[to]を更新できる
        for &(from, to) in edges.iter() {
            if v == from {
                dp[to] = dp[to].max(dp[v] + 1);
            }
        }
    }
    *dp.iter().max().unwrap()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_sample_passes() {
        let edges = vec![(0, 1), (1, 2), (0, 2)];
        let n = 3;
        assert_eq!(find_longest_path(&edges, n), 2);
    }

    #[test]
    fn test_straight_line_4_vertices() {
        // 0 -> 1 -> 2 -> 3
        // 最長パス長は 3
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let n = 4;
        assert_eq!(find_longest_path(&edges, n), 3);
    }

    #[test]
    fn test_diamond_dag() {
        // 0 -> 1 -> 3
        //  \-> 2 ->/
        // 最長パス長は 2
        let edges = vec![(0, 1), (1, 3), (0, 2), (2, 3)];
        let n = 4;
        assert_eq!(find_longest_path(&edges, n), 2);
    }

    #[test]
    fn test_path_with_shortcut() {
        // 0 -> 1 -> 2 -> 3 が最長で長さ 3
        // さらに 0 -> 2 のショートカットもある
        let edges = vec![(0, 1), (1, 2), (2, 3), (0, 2)];
        let n = 4;
        assert_eq!(find_longest_path(&edges, n), 3);
    }
}

#[cfg(test)]
mod tests_topological_sort {
    use super::*;

    fn assert_is_topological_order(order: &[usize], edges: &Edges, n: usize) {
        // 頂点数が一致していること
        assert_eq!(order.len(), n);

        // 各頂点がちょうど1回ずつ現れることを確認
        let mut pos = vec![None; n];
        for (i, &v) in order.iter().enumerate() {
            assert!(v < n, "vertex {} is out of range", v);
            assert!(pos[v].is_none(), "vertex {} appears more than once", v);
            pos[v] = Some(i);
        }
        assert!(pos.iter().all(|p| p.is_some()), "some vertices are missing");

        // すべての辺 from -> to について from が to より前にあること
        for &(from, to) in edges.iter() {
            let from_pos = pos[from].unwrap();
            let to_pos = pos[to].unwrap();
            assert!(
                from_pos < to_pos,
                "edge {} -> {} is violated: pos[{}]={}, pos[{}]={}",
                from,
                to,
                from,
                from_pos,
                to,
                to_pos
            );
        }
    }

    #[test]
    fn test_topological_sort_straight_line() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let n = 4;
        let order = topological_sort(&edges, n);
        assert_is_topological_order(&order, &edges, n);

        assert_eq!(order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_topological_sort_diamond() {
        let edges = vec![(0, 1), (0, 2), (1, 3), (2, 3)];
        let n = 4;
        let order = topological_sort(&edges, n);
        assert_is_topological_order(&order, &edges, n);
    }

    #[test]
    fn test_topological_sort_multiple_sources() {
        let edges = vec![(0, 2), (1, 2)];
        let n = 3;
        let order = topological_sort(&edges, n);
        assert_is_topological_order(&order, &edges, n);
    }

    #[test]
    fn test_topological_sort_disconnected_graph() {
        let edges = vec![(0, 1), (2, 3)];
        let n = 4;
        let order = topological_sort(&edges, n);
        assert_is_topological_order(&order, &edges, n);
    }

    #[test]
    fn test_topological_sort_no_edges() {
        let edges = vec![];
        let n = 5;
        let order = topological_sort(&edges, n);
        assert_is_topological_order(&order, &edges, n);

        assert_eq!(order, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_topological_sort_complex_dag() {
        let edges = vec![
            (0, 2),
            (0, 3),
            (1, 3),
            (1, 4),
            (2, 5),
            (3, 5),
            (4, 6),
            (5, 6),
        ];
        let n = 7;
        let order = topological_sort(&edges, n);
        assert_is_topological_order(&order, &edges, n);
    }
}
