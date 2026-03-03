// 重み付き有向グラフ（隣接リスト）
type Graph = Vec<Vec<Edge>>;

// 有向辺 (from, to, weight)
type Edge = (usize, usize, isize);

// 辺集合
type Edges = Vec<Edge>;

fn build_graph(n: usize, edges: &Edges) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for &(from, to, weight) in edges.iter() {
        g[from].push((from, to, weight));
    }
    g
}

// ベルマンフォード法: 負の閉路の検出
// sから到達不可能な場所にある負の閉路は検出しない
// n: 頂点数, g: グラフ, s: 始点
fn bellman_ford(n: usize, g: &Graph, s: usize) -> bool {
    const INF: isize = 1_isize << 60;

    let mut dist = vec![INF; n];
    dist[s] = 0;

    for i in 0..n {
        let mut updated = false;

        for v in 0..n {
            if dist[v] == INF {
                continue;
            }

            for &(_from, to, weight) in g[v].iter() {
                // 緩和が可能
                if dist[v] + weight < dist[to] {
                    dist[to] = dist[v] + weight;
                    updated = true;
                }
            }
        }

        // 緩和が行われなかった = 最短距離が確定
        if !updated {
            break;
        }

        if i == n - 1 && updated {
            // n-1回の緩和後も更新があった = 負の閉路が存在
            return true;
        }
    }
    false
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bellman_ford() {
        let n = 4;
        let edges: Edges = vec![(0, 1, 1), (1, 2, 1), (2, 0, -3), (1, 3, 1)];
        let g = build_graph(n, &edges);
        assert!(bellman_ford(n, &g, 0)); // 負の閉路が存在
    }

    #[test]
    fn neg_cycle_reachable_from_start() {
        // 0 -> 1 -> 2 -> 0 の閉路重みが 1 + 1 - 3 = -1（負）
        let n = 4;
        let edges: Edges = vec![(0, 1, 1), (1, 2, 1), (2, 0, -3), (1, 3, 1)];
        let g = build_graph(n, &edges);
        assert!(bellman_ford(n, &g, 0));
    }

    #[test]
    fn no_neg_cycle_simple() {
        // 負の閉路なし
        let n = 4;
        let edges: Edges = vec![(0, 1, 2), (1, 2, 2), (2, 3, 2)];
        let g = build_graph(n, &edges);
        assert!(!bellman_ford(n, &g, 0));
    }

    #[test]
    fn neg_cycle_exists_but_unreachable_from_start() {
        // 負の閉路 (2 <-> 3) はあるが、始点0から到達できない
        let n = 4;
        let edges: Edges = vec![
            (0, 1, 1),
            // 2 <-> 3 が負の閉路: 2->3 (-2), 3->2 (-2) で合計 -4
            (2, 3, -2),
            (3, 2, -2),
        ];
        let g = build_graph(n, &edges);
        assert!(!bellman_ford(n, &g, 0)); // 到達不能なので検出しない（仕様どおり）
    }

    #[test]
    fn neg_cycle_reachable_if_start_inside_component() {
        // 上と同じグラフだが、始点を 2 にすると負の閉路が検出される
        let n = 4;
        let edges: Edges = vec![(0, 1, 1), (2, 3, -2), (3, 2, -2)];
        let g = build_graph(n, &edges);
        assert!(bellman_ford(n, &g, 2));
    }

    #[test]
    fn single_vertex_negative_self_loop() {
        // 0->0 の自己ループが負なら、それ自体が負の閉路
        let n = 1;
        let edges: Edges = vec![(0, 0, -1)];
        let g = build_graph(n, &edges);
        assert!(bellman_ford(n, &g, 0));
    }

    #[test]
    fn zero_weight_cycle_is_not_negative() {
        // 0->1 (1), 1->0 (-1) で閉路重み 0：負ではない
        let n = 2;
        let edges: Edges = vec![(0, 1, 1), (1, 0, -1)];
        let g = build_graph(n, &edges);
        assert!(!bellman_ford(n, &g, 0));
    }

    #[test]
    fn has_negative_edge_but_no_negative_cycle() {
        // 負の辺はあるが、閉路が負にならない（そもそも閉路がない/負閉路がない）
        let n = 3;
        let edges: Edges = vec![(0, 1, -5), (1, 2, 2), (0, 2, 0)];
        let g = build_graph(n, &edges);
        assert!(!bellman_ford(n, &g, 0));
    }

    #[test]
    fn positive_cycle_is_not_negative() {
        // 閉路はあるが重みが正
        let n = 3;
        let edges: Edges = vec![(0, 1, 2), (1, 2, 2), (2, 0, 2)];
        let g = build_graph(n, &edges);
        assert!(!bellman_ford(n, &g, 0));
    }
}
