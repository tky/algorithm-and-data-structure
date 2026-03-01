pub type Vertex = usize;

type Graph = Vec<Vec<usize>>;

type Edge = (usize, usize);

type Edges = Vec<Edge>;

fn build_graph(n: usize, edges: &Edges) -> Graph {
    let mut g: Graph = vec![Vec::new(); n];
    for &(from, to) in edges.iter() {
        g[from].push(to);
    }
    g
}

pub fn find_directed_cycle(g: &Graph) -> Option<Vec<Vertex>> {
    let n = g.len();

    // 0 = unvisited, 1 = in current DFS stack (gray), 2 = finished (black)
    let mut state = vec![0u8; n];
    let mut parent = vec![usize::MAX; n];

    fn reconstruct_cycle(start: Vertex, mut v: Vertex, parent: &[Vertex]) -> Vec<Vertex> {
        let mut path = Vec::new();
        path.push(v);
        while v != start {
            v = parent[v];
            path.push(v);
        }
        path.reverse();
        path.push(start);
        path
    }

    for s in 0..n {
        if state[s] != 0 {
            continue;
        }

        state[s] = 1;
        parent[s] = usize::MAX;
        // 次に訪れる頂点と、その頂点の隣接リストのどこまで見たかをスタックに保存する
        let mut stack: Vec<(Vertex, usize)> = vec![(s, 0)];

        while !stack.is_empty() {
            // stackの一番上にある頂点と、その頂点の隣接リストのどこまで見たかの参照を取り出す
            // (stackから取り除いていないことに注意)
            let (v, i) = {
                let &(v, i) = stack.last().unwrap();
                (v, i)
            };

            // 隣接リストを全て見たら、この頂点はおしまい
            if i == g[v].len() {
                state[v] = 2;
                // ここで初めてstackからの取り除く
                stack.pop();
                continue;
            }

            // 次に訪れる頂点をスタックに保存する前に、隣接リストのどこまで見たかを更新する
            // これで次は同じ頂点を見たときに、次の隣接リストの頂点を見れるようになる
            stack.last_mut().unwrap().1 += 1;

            let to = g[v][i];
            match state[to] {
                0 => {
                    parent[to] = v;
                    state[to] = 1;
                    stack.push((to, 0));
                }
                1 => {
                    return Some(reconstruct_cycle(to, v, &parent));
                }
                _ => {}
            }
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
    fn test_find_directed_cycle() {
        let edges = vec![(0, 1), (1, 2), (2, 0), (1, 3)];
        let g = build_graph(4, &edges);
        let cycle = find_directed_cycle(&g);
        assert!(cycle.is_some());
        let cycle = cycle.unwrap();
        assert_eq!(cycle[0], cycle[cycle.len() - 1]);
        assert_eq!(cycle.len(), 4);
    }
}
