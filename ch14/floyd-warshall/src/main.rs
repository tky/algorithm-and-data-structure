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

fn floyd_warshall(n: usize, graph: &Graph) -> Vec<Vec<usize>> {
    const INF: usize = std::usize::MAX / 4;
    // dp[i][i][j]: 0,1,...,k-1のみを中継点として通って良い時のiからjへの最短経路
    let mut dp = vec![vec![vec![INF; n]; n]; n + 1];

    // 初期化
    // dp[0][i][j]
    // 0 : (i = j)
    // w: (i, j, w) ∈ graph
    // INF: それ以外
    for i in 0..n {
        for j in 0..n {
            if i == j {
                dp[0][i][j] = 0;
            } else if let Some(w) = weight(graph, i, j) {
                dp[0][i][j] = w;
            }
        }
    }

    // 漸化式
    // kを使わない場合
    // dp[k][i][j]
    // kを使う場合
    // dp[k][i][k] <- kを使ってiからkへの最短経路
    // dp[k][k][j] <- kを使ってkからjへの最短経路
    // dp[k][i][k] + dp[k][k][j] <- kを使ってiからjへの経路
    // より
    // dp[k+1][i][j] = min(dp[k][i][j], dp[k][i][k] + dp[k][k][j])
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let Some(min) = dp[k][i][k].checked_add(dp[k][k][j]) {
                    dp[k + 1][i][j] = min;
                }
                dp[k + 1][i][j] = dp[k + 1][i][j].min(dp[k][i][j]);
            }
        }
    }

    dp[n].clone()
}

fn weight(graph: &Graph, from: usize, to: usize) -> Option<usize> {
    graph[from].iter().find(|g| g.1 == to).map(|g| g.2)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floyd_warshall() {
        let n = 4;
        let edges: Edges = vec![(0, 1, 2), (0, 2, 5), (1, 2, 1), (1, 3, 3), (2, 3, 1)];
        let graph = build_graph(n, &edges);
        let dist = floyd_warshall(n, &graph);
        assert_eq!(dist[0][1], 2); // 0 -> 1
        assert_eq!(dist[0][2], 3); // 0 -> 1 -> 2
        assert_eq!(dist[0][3], 4); // 0 -> 1 -> 2 -> 3
        assert_eq!(dist[1][3], 2); // 1 -> 2 -> 3
        assert_eq!(dist[2][3], 1);
    }
}
