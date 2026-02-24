mod union_find;
use union_find::UnionFind;

type Query = (usize, usize, isize);

fn resolve(n: usize, _m: usize, queries: Vec<Query>) -> bool {
    let mut uf = UnionFind::new(n);
    // 橋を順次おなじグループとして併合していく
    // この時、併合の際に「重み」を考慮する必要がある。
    // 重みは「根からの差分」として管理する。
    for (l, r, d) in queries {
        if uf.same(l - 1, r - 1) {
            // すでに同一成分
            if uf.weight(r - 1) - uf.weight(l - 1) != d {
                // 同一成分で根からの距離が矛盾している場合はfalseを返す
                // 同一成分なのでweightの差分が計算できる
                // weightとは対象の要素が属しているグループの根からの距離
                // もしグループ違う場合、weightの差分は意味をなさない
                return false;
            }
        } else {
            uf.union(l - 1, r - 1, d);
        }
    }
    true
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{Query, resolve};

    #[test]
    fn test_resolve() {
        let n = 3;
        let m = 3;
        let queries: Vec<Query> = vec![(1, 2, 1), (2, 3, 1), (1, 3, 2)];
        assert!(resolve(n, m, queries));
    }
}
