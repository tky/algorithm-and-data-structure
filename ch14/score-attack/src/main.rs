// 有向辺 (from, to, weight)
type Edge = (usize, usize, i64);

// 辺集合
type Edges = Vec<Edge>;

type Graph = Vec<Vec<(usize, i64)>>;

fn build_graph(n: usize, edges: &Edges) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for &(from, to, weight) in edges.iter() {
        g[from].push((to, weight));
    }
    g
}

fn score_attack(n: usize, g: &Graph) -> Option<i64> {
    const NEG_INF: i64 = -(1_i64 << 60);
    let mut dist = vec![NEG_INF; n];
    dist[0] = 0;

    // Bellman-Ford法で始点から到達可能な頂点の最大スコアを求める
    // 通常のBellman-Ford法は「最短距離」を求めるが、ここでは「最大スコア」を求めたいので、更新条件を逆にしている。
    for _ in 0..n - 1 {
        let mut updated = false;
        for v in 0..n {
            if dist[v] == NEG_INF {
                continue;
            }

            for &(to, weight) in g[v].iter() {
                if dist[to] < dist[v] + weight {
                    dist[to] = dist[v] + weight;
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }

    // 始点0から到達可能な頂点のうち、正の閉路の影響を受けるかどうかを判定するため、さらにn回緩和処理を行う。
    // まず、前半だけで分かること
    // 前半の n-1 回の緩和が終わると、閉路を使わない経路での最大値は出そろう
    // それでもなお更新できる頂点があれば、その頂点は 正の閉路の影響を受けていることがわかる
    let mut bad = vec![false; n];
    for _ in 0..n {
        for v in 0..n {
            if dist[v] == NEG_INF {
                continue;
            }
            for &(to, weight) in &g[v] {
                let cand = dist[v] + weight;
                // 最大値が更新できるということは、正の閉路の影響を受けている
                if dist[to] < cand {
                    dist[to] = cand;
                    bad[to] = true;
                }
                if bad[v] {
                    bad[to] = true;
                }
            }
        }
    }
    if bad[n - 1] { None } else { Some(dist[n - 1]) }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: usize, edges: Edges) -> Option<i64> {
        let g = build_graph(n, &edges);
        score_attack(n, &g)
    }

    #[test]
    fn finite_simple_path() {
        // 0 -> 1 -> 3 が最善
        //
        // 0 --2--> 1 --3--> 3
        //  \--1--> 2 --1--> 3
        let edges = vec![(0, 1, 2), (1, 3, 3), (0, 2, 1), (2, 3, 1)];
        assert_eq!(run(4, edges), Some(5));
    }

    #[test]
    fn positive_cycle_reaches_goal() {
        // 1 <-> 2 に正の閉路があり、そこから 4 に行ける
        //
        // 0 -> 1 -> 2 -> 3 -> 4
        //      ^    |
        //      |____|
        let edges = vec![
            (0, 1, 1),
            (1, 2, 1),
            (2, 1, 1), // cycle total = +2
            (2, 3, 1),
            (3, 4, 1),
        ];
        assert_eq!(run(5, edges), None);
    }

    #[test]
    fn positive_cycle_does_not_reach_goal() {
        // 1 <-> 2 に正の閉路はあるが、ゴール 3 には行けない
        //
        // 0 -> 1 <-> 2
        //  \
        //   -> 3
        let edges = vec![
            (0, 1, 10),
            (1, 2, 1),
            (2, 1, 1), // cycle total = +2
            (0, 3, 5),
        ];
        assert_eq!(run(4, edges), Some(5));
    }

    #[test]
    fn large_negative_answer() {
        // -5_000_000_000 になるケース
        let edges = vec![
            (0, 1, -1_000_000_000),
            (1, 2, -1_000_000_000),
            (2, 3, -1_000_000_000),
            (3, 4, -1_000_000_000),
            (4, 5, -1_000_000_000),
        ];
        assert_eq!(run(6, edges), Some(-5_000_000_000));
    }

    #[test]
    fn choose_better_of_multiple_paths() {
        // 複数経路のうち最大値を選べるか
        //
        // 0 -> 1 -> 4 : 3 + 4 = 7
        // 0 -> 2 -> 4 : 10 + (-100) = -90
        // 0 -> 3 -> 4 : 2 + 2 = 4
        let edges = vec![
            (0, 1, 3),
            (1, 4, 4),
            (0, 2, 10),
            (2, 4, -100),
            (0, 3, 2),
            (3, 4, 2),
        ];
        assert_eq!(run(5, edges), Some(7));
    }

    #[test]
    fn zero_weight_cycle_is_not_infinite() {
        // 閉路があっても正の閉路でなければ inf ではない
        //
        // 1 <-> 2 の合計が 0
        let edges = vec![
            (0, 1, 2),
            (1, 2, 3),
            (2, 1, -3), // cycle total = 0
            (2, 3, 4),
        ];
        assert_eq!(run(4, edges), Some(9));
    }
}
