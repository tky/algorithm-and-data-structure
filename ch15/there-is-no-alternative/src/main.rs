mod union_find;

// 無向グラフ from, to, weight
type Edge = (usize, usize, i64);

type Edges = Vec<Edge>;

fn kruskal_with_indices(eges: &Edges, n: usize) -> (Vec<usize>, i64) {
    let mut uf = union_find::UnionFind::new(n);
    let mut total_weight = 0;
    let mut results = Vec::new();

    for (i, &(from, to, weight)) in eges.iter().enumerate() {
        if uf.union(from, to) {
            total_weight += weight;
            results.push(i);
        }
    }
    (results, total_weight)
}

// 辺を外した結果森になる可能性があるのでOptionで返す必要がある
fn kruskal(eges: &Edges, n: usize, skip_index: usize) -> Option<i64> {
    let mut uf = union_find::UnionFind::new(n);
    let mut total_weight = 0;
    let mut used = 0;

    for (i, &(from, to, weight)) in eges.iter().enumerate() {
        if i == skip_index {
            continue;
        }
        if uf.union(from, to) {
            total_weight += weight;
            used += 1;
            if used == n - 1 {
                return Some(total_weight);
            }
        }
    }
    None
}

// 最小全域木の辺を一つずつ外して、重みが変わるかどうかを確認する
// edgesはソートされているものとする
fn resolve(edges: &Edges, n: usize) -> Vec<usize> {
    let mut results = Vec::new();
    // まず全部の辺を最小全域木の辺と、重みを計算する
    let (candidates, total_weight) = kruskal_with_indices(edges, n);

    for i in candidates {
        let weight = kruskal(edges, n, i);
        // total_weightと同じ重みの最小全域木が作れるなら、この辺は必須ではない
        // 辺を外すことで重みが大きくなってしまうなら、この辺は必須
        if weight != Some(total_weight) {
            results.push(i);
        }
    }
    results
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(from: usize, to: usize, weight: i64) -> Edge {
        (from, to, weight)
    }

    #[test]
    fn unique_mst_all_edges_are_mandatory() {
        // 0-1(1), 1-2(2), 0-2(3)
        // MST は {0,1} で一意
        let edges: Edges = vec![e(0, 1, 1), e(1, 2, 2), e(0, 2, 3)];

        assert_eq!(resolve(&edges, 3), vec![0, 1]);
    }

    #[test]
    fn all_mst_edges_have_alternatives() {
        // 三角形の全辺が同じ重み
        // どの 2 辺を選んでも MST になるので、必須辺はない
        let edges: Edges = vec![e(0, 1, 1), e(1, 2, 1), e(0, 2, 1)];

        assert_eq!(resolve(&edges, 3), Vec::<usize>::new());
    }

    #[test]
    fn only_bridge_like_edge_is_mandatory() {
        // 0-1-2 が重み 1 の三角形、3 は 2 に重み 2 でしかつながっていない
        //
        // edges[3] = (2,3,2) は必ず必要
        // 三角形側の 3 辺はどれも代替可能
        let edges: Edges = vec![e(0, 1, 1), e(1, 2, 1), e(0, 2, 1), e(2, 3, 2)];

        assert_eq!(resolve(&edges, 4), vec![3]);
    }

    #[test]
    fn mixed_case_some_mandatory_some_not() {
        // 0-1(1), 1-2(1), 0-2(2), 2-3(2), 1-3(2)
        //
        // MST の最小重みは 4
        // 0-1, 1-2 は両方必須
        // 2-3 と 1-3 は互いに代替可能
        let edges: Edges = vec![
            e(0, 1, 1), // index 0
            e(1, 2, 1), // index 1
            e(0, 2, 2), // index 2
            e(2, 3, 2), // index 3
            e(1, 3, 2), // index 4
        ];

        assert_eq!(resolve(&edges, 4), vec![0, 1]);
    }

    #[test]
    fn square_cycle_has_no_mandatory_edges() {
        // 正方形の 4 辺がすべて重み 1
        // どの 3 辺でも MST
        let edges: Edges = vec![e(0, 1, 1), e(1, 2, 1), e(2, 3, 1), e(3, 0, 1)];

        assert_eq!(resolve(&edges, 4), Vec::<usize>::new());
    }

    #[test]
    fn tree_input_all_edges_are_mandatory() {
        // もともと辺が n-1 本しかない木
        // どの辺も外せないので全部必須
        let edges: Edges = vec![e(0, 1, 1), e(1, 2, 2), e(2, 3, 3)];

        assert_eq!(resolve(&edges, 4), vec![0, 1, 2]);
    }
}
