use proconio::{fastout, input, marker::Usize1};

pub struct AdjList {
    dim: usize,
    list: Vec<Vec<usize>>,
}

impl AdjList {
    pub fn new(d: usize) -> AdjList {
        AdjList {
            dim: d,
            list: vec![vec![]; d],
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

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        abz: [(Usize1, Usize1); n-1],
        px: [(Usize1, usize); q],
    }

    let mut adl = AdjList::new(n);
    for ab in abz {
        adl.add_undirected_edge(ab.0, ab.1);
    }

    let mut visited = vec![false; n];
    let mut cnt = vec![0; n];
    for query in px {
        cnt[query.0] += query.1;
    }

    let mut stack = vec![];
    stack.push(0);
    while !stack.is_empty() {
        let now = stack.pop().unwrap();
        if visited[now] {
            continue;
        }
        visited[now] = true;
        for i in 0..adl.list[now].len() {
            let nxt = adl.list[now][i];
            if !visited[nxt] {
                cnt[nxt] += cnt[now];
                stack.push(nxt);
            }
        }
    }

    for c in cnt {
        println!("{}", c);
    }
}
