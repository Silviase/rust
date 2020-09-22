use proconio::{input, marker::Usize1};

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

pub struct AdjList {
    dim: usize,
    list: Vec<Vec<usize>>,
}

impl AdjList {
    pub fn new(d: usize) -> AdjList {
        AdjList {
            dim: d,
            list: vec![vec![]],
        }
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.list.get_mut(from).unwrap().push(to);
    }

    pub fn add_undirected_edge(&mut self, from: usize, to: usize) {
        self.list.get_mut(from).unwrap().push(to);
        self.list.get_mut(to).unwrap().push(from);
    }

    pub fn out_edges(&mut self, vertex: usize) -> Vec<usize> {
        self.list.get_mut(vertex).unwrap().to_vec()
    }

    pub fn in_edges(&mut self, v: usize) -> Vec<usize> {
        let mut res = vec![];
        for l in 0..self.dim() {
            if let Some(list) = self.list.get(l) {
                if list.contains(&v) {
                    res.push(l);
                }
            }
        }

        res
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m]
    }

    let mut res = 0;
    for i in 0..m {
        // iは使わないもの
        let mut uf = UnionFind::new(n);
        for j in 0..m {
            if j != i {
                uf.union(edges[j].0, edges[j].1);
            }
        }

        // つなげたので連結性判定
        let mut f = true;
        for j in 0..n {
            f &= uf.find(j) == 0;
        }
        if !f {
            res += 1;
        }
    }
    println!("{}", res);
}
