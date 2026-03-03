// 重み付き有向グラフ（隣接リスト）
type Graph = Vec<Vec<Edge>>;

// 有向辺 (from, to, weight)
type Edge = (usize, usize, usize);

// 辺集合
type Edges = Vec<Edge>;

fn build_graph(n: usize, edges: &Edges) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for &(from, to, weight) in edges.iter() {
        g[from].push((from, to, weight));
    }
    g
}

// ダイクストラ
// graph: グラフ, n: 頂点数, start: 始点
fn dijkstra(graph: &Graph, n: usize, start: usize) -> Vec<usize> {
    const INF: usize = std::usize::MAX;
    let mut visited: Vec<bool> = vec![false; n];
    let mut dist: Vec<usize> = vec![INF; n];
    dist[start] = 0;

    for _ in 0..n {
        // 未訪問の頂点の中で、distが最小の頂点を探す
        let mut min_dist = INF;
        let mut min_v = 0;
        let mut found = false;
        for v in 0..n {
            if !visited[v] && dist[v] < min_dist {
                min_dist = dist[v];
                min_v = v;
                found = true;
            }
        }

        if !found {
            // すべての頂点が訪問されたか、到達不可能な頂点が残っている場合は終了
            break;
        }

        // 最小の頂点から隣接する頂点への距離を更新
        let dv = dist[min_v];
        for &(_from, to, weight) in graph[min_v].iter() {
            if visited[to] {
                continue;
            }
            if let Some(nd) = dv.checked_add(weight) {
                if nd < dist[to] {
                    dist[to] = nd;
                }
            }
        }
        visited[min_v] = true;
    }

    dist
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    // 既存のサンプル
    #[test]
    fn sample_graph() {
        let edges: Edges = vec![(0, 1, 4), (0, 2, 1), (1, 2, 2), (1, 3, 5), (2, 3, 8)];
        let graph = build_graph(4, &edges);
        let dist = dijkstra(&graph, 4, 0);
        assert_eq!(dist, vec![0, 4, 1, 9]);
    }

    // 到達不能頂点がある
    #[test]
    fn unreachable_vertices() {
        // 0 -> 1 はあるが、2,3 は孤立
        let edges: Edges = vec![(0, 1, 3)];
        let graph = build_graph(4, &edges);
        let dist = dijkstra(&graph, 4, 0);
        assert_eq!(dist[0], 0);
        assert_eq!(dist[1], 3);
        assert_eq!(dist[2], usize::MAX);
        assert_eq!(dist[3], usize::MAX);
    }

    // 0重み辺があっても正しく動く
    #[test]
    fn zero_weight_edges() {
        // 0->1(0), 1->2(0), 0->2(5) なので dist[2]=0
        let edges: Edges = vec![(0, 1, 0), (1, 2, 0), (0, 2, 5)];
        let graph = build_graph(3, &edges);
        let dist = dijkstra(&graph, 3, 0);
        assert_eq!(dist, vec![0, 0, 0]);
    }

    // 複数経路があるとき短い方を選ぶ
    #[test]
    fn choose_shorter_path_over_direct() {
        // 0->2(10) より 0->1(2)->2(2)=4 が短い
        let edges: Edges = vec![(0, 2, 10), (0, 1, 2), (1, 2, 2)];
        let graph = build_graph(3, &edges);
        let dist = dijkstra(&graph, 3, 0);
        assert_eq!(dist, vec![0, 2, 4]);
    }

    // 重複辺（同じ from,to で重みが違う）があっても最小を取れる
    #[test]
    fn parallel_edges_take_min() {
        // 0->1 が 10 と 3 の2本
        let edges: Edges = vec![(0, 1, 10), (0, 1, 3), (1, 2, 4)];
        let graph = build_graph(3, &edges);
        let dist = dijkstra(&graph, 3, 0);
        assert_eq!(dist, vec![0, 3, 7]);
    }

    // 有向性：逆向きがなければ届かない
    #[test]
    fn directed_graph_asymmetric_reachability() {
        // 1->0 はあるが 0->1 はない
        let edges: Edges = vec![(1, 0, 1)];
        let graph = build_graph(2, &edges);

        let dist_from_0 = dijkstra(&graph, 2, 0);
        assert_eq!(dist_from_0, vec![0, usize::MAX]);

        let dist_from_1 = dijkstra(&graph, 2, 1);
        assert_eq!(dist_from_1, vec![1, 0]);
    }

    // start が 0 以外でも正しい
    #[test]
    fn start_not_zero() {
        // 2->0(5), 2->1(1), 1->0(1) なので start=2 の dist[0]=2
        let edges: Edges = vec![(2, 0, 5), (2, 1, 1), (1, 0, 1)];
        let graph = build_graph(3, &edges);
        let dist = dijkstra(&graph, 3, 2);
        assert_eq!(dist, vec![2, 1, 0]);
    }

    // 自己ループがあっても（非負なら）結果に影響しない
    #[test]
    fn self_loop_non_negative() {
        // 0->0(0), 0->1(2)
        let edges: Edges = vec![(0, 0, 0), (0, 1, 2)];
        let graph = build_graph(2, &edges);
        let dist = dijkstra(&graph, 2, 0);
        assert_eq!(dist, vec![0, 2]);
    }

    // サイクルがあっても（非負なら）OK
    #[test]
    fn non_negative_cycle() {
        // 0->1(1), 1->2(1), 2->1(1), 2->3(1)
        let edges: Edges = vec![(0, 1, 1), (1, 2, 1), (2, 1, 1), (2, 3, 1)];
        let graph = build_graph(4, &edges);
        let dist = dijkstra(&graph, 4, 0);
        assert_eq!(dist, vec![0, 1, 2, 3]);
    }

    // 複数成分：別成分は INF のまま
    #[test]
    fn multiple_components() {
        // 成分A: 0->1->2, 成分B: 3->4
        let edges: Edges = vec![(0, 1, 1), (1, 2, 1), (3, 4, 1)];
        let graph = build_graph(5, &edges);
        let dist = dijkstra(&graph, 5, 0);
        assert_eq!(dist[0], 0);
        assert_eq!(dist[1], 1);
        assert_eq!(dist[2], 2);
        assert_eq!(dist[3], usize::MAX);
        assert_eq!(dist[4], usize::MAX);
    }

    // --- オーバーフロー耐性テスト（checked_add 版なら通る） ---
    // 元実装だと release で壊れたり debug で panic する可能性があります。
    #[test]
    fn overflow_safety_expected_with_checked_add() {
        // 0->1 が非常に大きい、1->2 がさらに大きい
        // 安全実装なら dist[2] は更新されず INF のまま（加算がオーバーフローするため）
        let big = usize::MAX - 10;
        let edges: Edges = vec![(0, 1, big), (1, 2, 100)];
        let graph = build_graph(3, &edges);
        let dist = dijkstra(&graph, 3, 0);

        assert_eq!(dist[0], 0);
        assert_eq!(dist[1], big);
        // checked_add で溢れを弾くなら INF のまま
        // （元実装だと wrap して小さくなり、誤って更新される可能性あり）
        assert_eq!(dist[2], usize::MAX);
    }
}
