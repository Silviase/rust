pub struct UnionFind {
    dim: usize,
    par: Vec<usize>,
}

impl UnionFind {
    pub fn new(d: usize) -> UnionFind {
        let mut list = vec![0; d];
        for i in 0..d {
            list[i] = i;
        }
        UnionFind { dim: d, par: list }
    }

    pub fn dim(self) -> usize {
        self.dim
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root > y_root {
            self.union(y, x);
        } else if x_root < y_root {
            self.par[y_root] = x_root;
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.find(self.par[x]);
        self.par[x]
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        return self.find(x) == self.find(y);
    }
}
