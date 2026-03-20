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
