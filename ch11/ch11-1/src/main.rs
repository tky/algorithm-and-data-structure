use petgraph::unionfind::UnionFind;

// 隣接頂点の集合
type Edge = (usize, usize);

// グラフの橋の数を数える
fn resolve(n: usize, edges: &Vec<Edge>) -> usize {
    let mut result = 0;

    let len = edges.len();
    // iを取り除いた時にrootがいくつあるかをカウントする
    for skip in 0..len {
        let mut uf = UnionFind::<usize>::new(n);
        for (i, &(x, y)) in edges.iter().enumerate() {
            if i == skip {
                continue;
            }
            uf.union(x, y);
        }

        // 0番目の頂点が全ての頂点と繋がっているかどうか
        // 一つでも繋がっていない点があれば連結グラフではないのでiは橋である
        let r0 = uf.find_mut(0);
        let connected = (1..n).all(|v| uf.find_mut(v) == r0);
        if !connected {
            result += 1;
        }
    }
    result
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolve() {
        let edges = vec![(0, 2), (1, 6), (2, 3), (3, 4), (3, 5), (4, 5), (5, 6)];
        let n = 7; // 頂点の数
        assert_eq!(resolve(n, &edges), 4);
    }
}
