use proconio::{fastout, input};

pub struct UnionFind {
    n: usize,
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn get_all_size(&self) -> usize {
        self.n
    }

    pub fn new(n: usize) -> UnionFind {
        let mut par = vec![0; n];
        for i in 0..n {
            par[i] = i;
        }
        UnionFind {
            n,
            par,
            size: vec![1; n],
        }
    }

    pub fn root(&mut self, idx: usize) -> usize {
        if self.par[idx] == idx {
            idx
        } else {
            self.par[idx] = self.root(self.par[idx]);
            self.par[idx]
        }
    }

    pub fn size(&self, idx: usize) -> usize {
        self.size[idx]
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
        x = self.root(x);
        y = self.root(y);
        if x == y {
            false
        } else {
            if self.size[x] < self.size[y] {
                let tmp = x;
                x = y;
                y = tmp;
            }
            self.size[x] += self.size[y];
            self.par[y] = x;
            true
        }
    }
}

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        pab: [usize; 3 * q],
    }

    let mut uf = UnionFind::new(n);

    for i in 0..q {
        if pab[3 * i] == 0 {
            uf.union(pab[3 * i + 1], pab[3 * i + 2]);
        } else {
            if uf.same(pab[3 * i + 1], pab[3 * i + 2]) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
