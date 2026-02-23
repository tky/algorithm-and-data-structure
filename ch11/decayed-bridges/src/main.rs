use petgraph::unionfind::UnionFind;

type Edge = (usize, usize);

fn main() {
    println!("Hello, world!");
}

fn resolve(n: usize, edges: Vec<Edge>) -> Vec<usize> {
    let mut ans = Vec::new();

    for i in 1..=edges.len() {
        // とりあえず毎回UnionFindを新規で作ってみる
        let mut uf = UnionFind::new(n);
        for &(x, y) in edges.iter().skip(i) {
            uf.union(x, y);
        }
        ans.push(inconvenience(n, &uf));
    }
    ans
}

fn inconvenience(n: usize, uf: &UnionFind<usize>) -> usize {
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            if uf.find(i) != uf.find(j) {
                ans += 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let edges = vec![(0, 1), (2, 3), (0, 2), (1, 2), (0, 3)];
        let result = resolve(4, edges);
        assert_eq!(result, vec![0, 0, 4, 5, 6]);
    }

    #[test]
    fn test_no_edges_n4_m0() {
        // M=0 のとき resolve は空配列を返す（あなたの for ループ仕様）
        let edges: Vec<(usize, usize)> = vec![];
        let result = resolve(4, edges);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_n1_single_vertex() {
        // N=1 なら、どんな途中状態でも非連結ペアは常に 0
        let edges = vec![(0, 0), (0, 0)]; // ※ AtCoder本番では自己ループは来ないが、実装耐性チェック
        let result = resolve(1, edges);
        assert_eq!(result, vec![0, 0]);
    }

    #[test]
    fn test_n2_single_edge() {
        // N=2, 辺が1本だけ
        // i=1: skip(1) => 辺なし => 非連結ペアは 1
        let edges = vec![(0, 1)];
        let result = resolve(2, edges);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_chain_n3_edges2() {
        // 0-1-2 の鎖
        // i=1: 残り (1,2) => 成分 {1,2},{0} => 非連結ペア 2
        // i=2: 残りなし => 全孤立 => 非連結ペア 3
        let edges = vec![(0, 1), (1, 2)];
        let result = resolve(3, edges);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_star_n4_center0() {
        // 0 を中心に星
        // edges = (0,1)(0,2)(0,3)
        // i=1: 残り (0,2)(0,3) => 成分 {0,2,3},{1} => 非連結 3
        // i=2: 残り (0,3)       => 成分 {0,3},{1},{2} => 非連結 5
        // i=3: 残りなし         => 全孤立 => 非連結 6
        let edges = vec![(0, 1), (0, 2), (0, 3)];
        let result = resolve(4, edges);
        assert_eq!(result, vec![3, 5, 6]);
    }

    #[test]
    fn test_order_matters_same_graph() {
        // グラフとしては同じ（0-1 と 1-2）だが、順番が違うと途中の答えが変わることを確認
        // A: (0,1),(1,2) は test_chain_n3_edges2 と同じで [2,3]
        // B: (1,2),(0,1) の場合：
        //   i=1: 残り (0,1) => 成分 {0,1},{2} => 非連結 2
        //   i=2: 残りなし  => 3
        let edges = vec![(1, 2), (0, 1)];
        let result = resolve(3, edges);
        assert_eq!(result, vec![2, 3]);
    }
}
