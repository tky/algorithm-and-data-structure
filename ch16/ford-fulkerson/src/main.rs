use std::usize;

#[derive(Clone, Debug)]
// 現在の残余グラフを直接表しているデータ構造
// 残余グラフを別に持つのではなく、逆辺を使って元のグラフを更新していく
struct Edge {
    to: usize,
    cap: usize,
    rev: usize, // 逆辺のインデックス
}

type Graph = Vec<Vec<Edge>>;

// 入力用の辺集合: (from, to, capacity)
type Edges = Vec<(usize, usize, usize)>;

fn add_edge(g: &mut Graph, from: usize, to: usize, cap: usize) {
    let rev_to = g[to].len();
    let rev_from = g[from].len();

    g[from].push(Edge {
        to,
        cap,
        rev: rev_to,
    });

    g[to].push(Edge {
        to: from,
        cap: 0,
        rev: rev_from,
    });
}

fn build_graph(n: usize, edges: &Edges) -> Graph {
    let mut g = vec![Vec::new(); n];
    for &(from, to, cap) in edges.iter() {
        add_edge(&mut g, from, to, cap);
    }
    g
}

// v: current vertex
// t: target vertex
// f: current flow (the minimum capacity along the path so far)
fn dfs(v: usize, t: usize, f: usize, used: &mut [bool], g: &mut Graph) -> usize {
    if v == t {
        return f;
    }

    used[v] = true;

    for i in 0..g[v].len() {
        let cap = g[v][i].cap;
        let to = g[v][i].to;

        if cap == 0 || used[to] {
            continue;
        }

        let d = dfs(to, t, f.min(cap), used, g);

        if d > 0 {
            let rev = g[v][i].rev;
            // 正辺の容量を減らす
            g[v][i].cap -= d;
            // 逆辺の容量を増やす
            g[to][rev].cap += d;
            return d;
        }
    }
    0
}

// s から t への最大流
fn ford_fulkerson(g: &mut Graph, s: usize, t: usize) -> usize {
    let mut flow = 0;

    loop {
        let mut used = vec![false; g.len()];
        let d = dfs(s, t, usize::MAX, &mut used, g);
        if d == 0 {
            break;
        }
        flow += d;
    }

    flow
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_edge() {
        let edges = vec![(0, 1, 7)];
        let mut g = build_graph(2, &edges);

        let flow = ford_fulkerson(&mut g, 0, 1);
        assert_eq!(flow, 7);
    }

    #[test]
    fn sample_graph() {
        let edges = vec![(0, 1, 3), (0, 2, 2), (1, 2, 1), (1, 3, 2), (2, 3, 3)];
        let mut g = build_graph(4, &edges);

        let flow = ford_fulkerson(&mut g, 0, 3);
        assert_eq!(flow, 5);
    }

    #[test]
    fn no_path() {
        let edges = vec![(0, 1, 3), (2, 3, 4)];
        let mut g = build_graph(4, &edges);

        let flow = ford_fulkerson(&mut g, 0, 3);
        assert_eq!(flow, 0);
    }

    #[test]
    fn parallel_edges() {
        let edges = vec![(0, 1, 3), (0, 1, 4)];
        let mut g = build_graph(2, &edges);

        let flow = ford_fulkerson(&mut g, 0, 1);
        assert_eq!(flow, 7);
    }

    #[test]
    fn uses_reverse_edge_in_residual_graph() {
        // 最初に 0 -> 1 -> 2 -> 3 に 1 流してしまうと、
        // 最大流 2 を達成するには残余グラフの逆辺を使って
        // 0 -> 2 -> 1 -> 3 を見つける必要がある
        let edges = vec![(0, 1, 1), (0, 2, 1), (1, 2, 1), (1, 3, 1), (2, 3, 1)];
        let mut g = build_graph(4, &edges);

        let flow = ford_fulkerson(&mut g, 0, 3);
        assert_eq!(flow, 2);
    }
}
