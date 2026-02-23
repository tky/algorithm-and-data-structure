mod union_find;
use std::collections::HashMap;

use union_find::UnionFind;

type Edge = (usize, usize);

// 愚直に実装
fn resolve(n: usize, roads: Vec<Edge>, rails: Vec<Edge>) -> Vec<usize> {
    let mut uf_road = UnionFind::new(n);
    let mut uf_rail = UnionFind::new(n);

    for (a, b) in roads {
        uf_road.union(a, b);
    }
    for (a, b) in rails {
        uf_rail.union(a, b);
    }

    let mut result = Vec::new();

    for i in 0..n {
        let mut count = 0;
        for j in 0..n {
            if uf_road.same(i, j) && uf_rail.same(i, j) {
                count += 1;
            }
        }
        result.push(count);
    }
    result
}

// 各都市について代表元のペアをキーにして、同じペアの都市数を数える
// 都市iと都市jが道路でも電車でも連結しているということは
// 電車、道路のUnionFindで同じ代表元を持つということなので、ペアをキーにして数えると効率的に求められる
fn resolve2(n: usize, roads: Vec<Edge>, rails: Vec<Edge>) -> Vec<usize> {
    let mut uf_road = UnionFind::new(n);
    let mut uf_rail = UnionFind::new(n);

    for (a, b) in roads {
        uf_road.union(a, b);
    }
    for (a, b) in rails {
        uf_rail.union(a, b);
    }

    let mut pair_count = HashMap::new();
    for i in 0..n {
        let road = uf_road.find(i);
        let rail = uf_rail.find(i);
        pair_count
            .entry((road, rail))
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    let mut result = Vec::new();
    for i in 0..n {
        let road = uf_road.find(i);
        let rail = uf_rail.find(i);
        result.push(pair_count[&(road, rail)]);
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
    fn test_sample1() {
        let n = 4;
        let roads = vec![(0, 1), (1, 2), (2, 3)];
        let rails = vec![(1, 2)];
        let expected = vec![1, 2, 2, 1];
        assert_eq!(resolve(n, roads.clone(), rails.clone()), expected);
        assert_eq!(resolve2(n, roads.clone(), rails.clone()), expected);
    }

    #[test]
    fn test_sample2() {
        let n = 4;
        let roads = vec![(0, 1), (1, 2)];
        let rails = vec![(0, 3), (1, 2)];
        let expected = vec![1, 2, 2, 1];
        assert_eq!(resolve(n, roads.clone(), rails.clone()), expected);
        assert_eq!(resolve2(n, roads.clone(), rails.clone()), expected);
    }
}
