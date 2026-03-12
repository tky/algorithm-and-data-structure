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

// BFSを用いてsからtへのパスが存在するかを判定する
fn can_reach(graph: &Graph, s: usize, t: usize) -> bool {
    let mut queue = std::collections::VecDeque::new();
    let mut visited = vec![false; graph.len()];
    // 初期処理
    queue.push_back(s);
    visited[s] = true;

    while let Some(v) = queue.pop_front() {
        if v == t {
            return true;
        }
        for &to in graph[v].iter() {
            if !visited[to] {
                visited[to] = true;
                queue.push_back(to);
            }
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
    fn test_start_equals_goal() {
        let n = 1;
        let edges: Edges = vec![];
        let g = build_graph(n, &edges);

        assert!(can_reach(&g, 0, 0));
    }

    #[test]
    fn test_direct_edge() {
        let n = 2;
        let edges: Edges = vec![(0, 1)];
        let g = build_graph(n, &edges);

        assert!(can_reach(&g, 0, 1));
    }

    #[test]
    fn test_no_path() {
        let n = 3;
        let edges: Edges = vec![(0, 1)];
        let g = build_graph(n, &edges);

        assert!(!can_reach(&g, 0, 2));
    }

    #[test]
    fn test_path_through_intermediate_vertices() {
        let n = 4;
        let edges: Edges = vec![(0, 1), (1, 2), (2, 3)];
        let g = build_graph(n, &edges);

        assert!(can_reach(&g, 0, 3));
    }

    #[test]
    fn test_cycle_reachable() {
        let n = 4;
        let edges: Edges = vec![(0, 1), (1, 2), (2, 1), (2, 3)];
        let g = build_graph(n, &edges);

        assert!(can_reach(&g, 0, 3));
    }

    #[test]
    fn test_cycle_but_goal_unreachable() {
        let n = 5;
        let edges: Edges = vec![(0, 1), (1, 2), (2, 1), (3, 4)];
        let g = build_graph(n, &edges);

        assert!(!can_reach(&g, 0, 4));
    }

    #[test]
    fn test_self_loop() {
        let n = 2;
        let edges: Edges = vec![(0, 0), (0, 1)];
        let g = build_graph(n, &edges);

        assert!(can_reach(&g, 0, 1));
    }

    #[test]
    fn test_direction_matters_in_directed_graph() {
        let n = 2;
        let edges: Edges = vec![(0, 1)];
        let g = build_graph(n, &edges);

        assert!(can_reach(&g, 0, 1));
        assert!(!can_reach(&g, 1, 0));
    }

    #[test]
    fn test_multiple_routes() {
        let n = 5;
        let edges: Edges = vec![(0, 1), (1, 4), (0, 2), (2, 3), (3, 4)];
        let g = build_graph(n, &edges);

        assert!(can_reach(&g, 0, 4));
    }

    #[test]
    fn test_disconnected_graph() {
        let n = 6;
        let edges: Edges = vec![(0, 1), (1, 2), (3, 4)];
        let g = build_graph(n, &edges);

        assert!(!can_reach(&g, 0, 5));
        assert!(can_reach(&g, 0, 2));
        assert!(can_reach(&g, 3, 4));
    }
}
