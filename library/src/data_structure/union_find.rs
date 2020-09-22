pub struct UnionFind {
    n: usize,
    par: [usize],
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(&self, n: usize) {
        self.n = n;
        self.par = vec![usize; n];
        for i in 0..n {
            par[i] = i;
            size[i] = 1;
        }
    }

    pub fn root(&self, idx: usize) -> usize {
        if (par[idx] == idx) {
            idx
        } else {
            par[idx] = self.root(par[idx])
        }
    }

    pub fn size(&self, idx: usize) -> usize {
        self.size[idx]
    }

    pub fn same(&self, x: usize, y: usize) -> bool {
        root(x) == root(y)
    }

    pub fn union(&self, x: usize, y: usize) -> bool {
        x = root(x);
        y = root(y);
        if x == y {
            false
        }
        if (size[x] < size[y]) {
            let tmp = x;
            y = x;
            x = tmp;
        }
        size[x] += size[y];
        par[y] = x;

        true
    }
}
