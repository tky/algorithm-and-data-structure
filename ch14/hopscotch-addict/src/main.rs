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

// 頂点だけでなく、頂点+状態を管理することで3の倍数で到達できるかどうかを判定できるようになる
fn resolve(g: &Graph, n: usize, start: usize, goal: usize) -> Option<usize> {
    const INF: usize = std::usize::MAX;
    // dist[i][j] := 頂点iに状態jで到達するための最短距離
    // jはmod3
    // 初期値はINFを入れておく
    let mut dist: Vec<Vec<usize>> = vec![vec![INF; 3]; n];
    // 頂点の番号と状態(0,1,2)を保存するqueue
    let mut queue: std::collections::VecDeque<(usize, usize)> = std::collections::VecDeque::new();

    queue.push_back((start, 0));
    dist[start][0] = 0;

    while let Some((v, state)) = queue.pop_front() {
        if v == goal && state == 0 {
            return Some(dist[v][state]);
        }
        for &n in g[v].iter() {
            let next_state = (state + 1) % 3;

            if dist[n][next_state] != INF {
                continue;
            }
            dist[n][next_state] = dist[v][state] + 1;
            queue.push_back((n, next_state));
        }
    }
    None
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_equals_goal() {
        let n = 1;
        let edges: Edges = vec![];
        let g = build_graph(n, &edges);

        assert_eq!(resolve(&g, n, 0, 0), Some(0));
    }

    #[test]
    fn test_simple_three_step_path() {
        // 0 -> 1 -> 2 -> 3
        let n = 4;
        let edges: Edges = vec![(0, 1), (1, 2), (2, 3)];
        let g = build_graph(n, &edges);

        assert_eq!(resolve(&g, n, 0, 3), Some(3));
    }

    #[test]
    fn test_two_step_path_is_not_goal() {
        // 0 -> 1 -> 2 は 2 辺なので不可
        let n = 3;
        let edges: Edges = vec![(0, 1), (1, 2)];
        let g = build_graph(n, &edges);

        assert_eq!(resolve(&g, n, 0, 2), None);
    }

    #[test]
    fn test_unreachable_goal() {
        let n = 4;
        let edges: Edges = vec![(0, 1), (1, 2)];
        let g = build_graph(n, &edges);

        assert_eq!(resolve(&g, n, 0, 3), None);
    }

    #[test]
    fn test_cycle_adjusts_mod3_and_makes_goal_reachable() {
        // 0=S, 1=A, 2=B, 3=T
        //
        // S -> A -> T だと 2 辺で不可
        // A <-> B の長さ 2 の閉路を 2 周して
        // S -> A -> B -> A -> B -> A -> T = 6 辺で可
        let n = 4;
        let edges: Edges = vec![(0, 1), (1, 2), (2, 1), (1, 3)];
        let g = build_graph(n, &edges);

        assert_eq!(resolve(&g, n, 0, 3), Some(6));
    }

    #[test]
    fn test_multiple_paths_choose_shortest_valid_one() {
        // 0 -> 1 -> 2 -> 5   (3 辺)
        // 0 -> 3 -> 4 -> 1 -> 2 -> 5 (5 辺)
        // 最短の有効経路は 3
        let n = 6;
        let edges: Edges = vec![(0, 1), (1, 2), (2, 5), (0, 3), (3, 4), (4, 1)];
        let g = build_graph(n, &edges);

        assert_eq!(resolve(&g, n, 0, 5), Some(3));
    }

    #[test]
    fn test_reachable_after_cycle_adjustment() {
        let n = 3;
        let edges: Edges = vec![(0, 1), (1, 2), (2, 1)];
        let g = build_graph(n, &edges);

        assert_eq!(resolve(&g, n, 0, 2), Some(6));
    }

    #[test]
    fn test_self_loop_can_adjust_mod3() {
        // 0 -> 1, 1 -> 1, 1 -> 2
        // 0->1->1->2 = 3 辺で到達可能
        let n = 3;
        let edges: Edges = vec![(0, 1), (1, 1), (1, 2)];
        let g = build_graph(n, &edges);

        assert_eq!(resolve(&g, n, 0, 2), Some(3));
    }

    #[test]
    fn test_atcoder_sample_1() {
        // ABC132 E sample 1
        // 1-indexed:
        // 4 4
        // 1 2
        // 2 3
        // 3 4
        // 4 2
        // 1 4
        //
        // 0-indexed に直す
        let n = 4;
        let edges: Edges = vec![(0, 1), (1, 2), (2, 3), (3, 1)];
        let g = build_graph(n, &edges);

        assert_eq!(resolve(&g, n, 0, 3), Some(3));
    }
    #[test]
    fn test_reachable_only_in_two_steps_and_no_way_to_adjust() {
        let n = 3;
        let edges: Edges = vec![(0, 1), (1, 2)];
        let g = build_graph(n, &edges);

        assert_eq!(resolve(&g, n, 0, 2), None);
    }
}
