#[derive(Clone, Debug)]
// 重みつきUnion-Find
// それぞれの要素は根からの距離を重みとしてもつ
pub struct UnionFind {
    // root: -(component size), non-root: parent index
    parent_or_size: Vec<isize>,
    diff_weight: Vec<isize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent_or_size: vec![-1; n],
            diff_weight: vec![0; n],
        }
    }

    pub fn find(&mut self, a: usize) -> usize {
        let p = self.parent_or_size[a];
        if p < 0 {
            return a;
        }
        let root = self.find(p as usize);
        self.parent_or_size[a] = root as isize;
        self.diff_weight[a] += self.diff_weight[p as usize];
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
    pub fn union(&mut self, a: usize, b: usize, w: isize) -> bool {
        // 制約: x_b - x_a = w を満たすように併合する
        let mut x = self.find(a);
        let mut y = self.find(b);

        let wa = self.weight(a); // pos[a] - pos[x]
        let wb = self.weight(b); // pos[b] - pos[y]

        if x == y {
            return false;
        }

        // y を x の子にする場合に必要な根差分:
        // pos[b] - pos[a] = wを満たしたい
        // pos[b] - pos[y] + pos[y] - pos[x] + pos[x] - pos[a] = w
        let mut t = w + wa - wb;

        // union by size: x を大きい方にする
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
            // 親子が逆転するので符号反転
            t = -t;
        }

        self.parent_or_size[x] += self.parent_or_size[y]; // sizes are negative
        self.parent_or_size[y] = x as isize;
        // ここで保存している値は親(=x)からの距離(根からの距離ではない）
        self.diff_weight[y] = t;
        true
    }

    pub fn weight(&mut self, a: usize) -> isize {
        self.find(a);
        self.diff_weight[a]
    }
}
