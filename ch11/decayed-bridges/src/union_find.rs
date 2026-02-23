#[derive(Clone, Debug)]
pub struct UnionFind {
    // root: -(component size), non-root: parent index
    parent_or_size: Vec<isize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent_or_size: vec![-1; n],
        }
    }

    pub fn find(&mut self, a: usize) -> usize {
        let p = self.parent_or_size[a];
        if p < 0 {
            return a;
        }
        let root = self.find(p as usize);
        self.parent_or_size[a] = root as isize;
        root
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn size(&mut self, a: usize) -> usize {
        let r = self.find(a);
        (-self.parent_or_size[r]) as usize
    }

    /// Union the components containing a and b.
    /// Returns true if merged (they were different), false otherwise.
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let mut x = self.find(a);
        let mut y = self.find(b);
        if x == y {
            return false;
        }
        // union by size: ensure x has larger size
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y]; // sizes are negative
        self.parent_or_size[y] = x as isize;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn test_new_all_singletons() {
        let mut uf = UnionFind::new(5);
        for i in 0..5 {
            assert!(uf.same(i, i));
            assert_eq!(uf.size(i), 1);
            assert_eq!(uf.find(i), i);
        }
        assert!(!uf.same(0, 1));
        assert!(!uf.same(3, 4));
    }

    #[test]
    fn test_union_returns_bool() {
        let mut uf = UnionFind::new(4);

        // first time merges
        assert!(uf.union(0, 1));
        assert!(uf.same(0, 1));

        // second time (already same) => false
        assert!(!uf.union(0, 1));
        assert!(uf.same(0, 1));
    }

    #[test]
    fn test_size_after_unions_chain() {
        let mut uf = UnionFind::new(5);

        uf.union(0, 1);
        assert_eq!(uf.size(0), 2);
        assert_eq!(uf.size(1), 2);

        uf.union(1, 2);
        assert_eq!(uf.size(0), 3);
        assert_eq!(uf.size(2), 3);

        uf.union(3, 4);
        assert_eq!(uf.size(3), 2);

        // merge two components: {0,1,2} and {3,4}
        uf.union(2, 3);
        assert_eq!(uf.size(0), 5);
        assert_eq!(uf.size(4), 5);

        // now all same
        for i in 0..5 {
            assert!(uf.same(0, i));
            assert_eq!(uf.size(i), 5);
        }
    }

    #[test]
    fn test_find_idempotent_and_compression_safe() {
        let mut uf = UnionFind::new(6);

        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(3, 4);
        uf.union(4, 5);

        // calling find repeatedly shouldn't change equivalence
        let r1 = uf.find(2);
        let r2 = uf.find(2);
        assert_eq!(r1, r2);

        assert!(uf.same(0, 2));
        assert!(!uf.same(0, 3));

        // merge components and re-check roots/sizes
        uf.union(2, 3);
        assert!(uf.same(0, 5));
        assert_eq!(uf.size(0), 6);

        // roots are consistent across all nodes
        let r = uf.find(0);
        for i in 1..6 {
            assert_eq!(uf.find(i), r);
        }
    }

    #[test]
    fn test_multiple_components_sizes() {
        let mut uf = UnionFind::new(10);

        // component A: 0-1-2-3 (size 4)
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);

        // component B: 4-5 (size 2)
        uf.union(4, 5);

        // component C: 6-7-8 (size 3)
        uf.union(6, 7);
        uf.union(7, 8);

        // singleton: 9 (size 1)

        for v in 0..4 {
            assert_eq!(uf.size(v), 4);
        }
        for v in 4..=5 {
            assert_eq!(uf.size(v), 2);
        }
        for v in 6..=8 {
            assert_eq!(uf.size(v), 3);
        }
        assert_eq!(uf.size(9), 1);

        assert!(!uf.same(0, 4));
        assert!(!uf.same(5, 6));
        assert!(!uf.same(8, 9));
    }
}
