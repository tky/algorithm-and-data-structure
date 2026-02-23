mod union_find;
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::resolve;

    #[test]
    fn test_sample1() {
        let n = 4;
        let roads = vec![(0, 1), (1, 2), (2, 3)];
        let rails = vec![(1, 2)];
        let expected = vec![1, 2, 2, 1];
        assert_eq!(resolve(n, roads, rails), expected);
    }

    #[test]
    fn test_sample2() {
        let n = 4;
        let roads = vec![(0, 1), (1, 2)];
        let rails = vec![(0, 3), (1, 2)];
        let expected = vec![1, 2, 2, 1];
        assert_eq!(resolve(n, roads, rails), expected);
    }
}
