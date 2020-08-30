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
