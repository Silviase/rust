use proconio::marker::Usize1;
use proconio::{fastout, input};
use std::collections::VecDeque;

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
        match self.list.get_mut(from) {
            Some(v) => v.push(to),
            None => self.list.insert(from, vec![to]),
        }
    }

    pub fn add_undirected_edge(&mut self, from: usize, to: usize) {
        self.add_edge(from, to);
        self.add_edge(to, from);
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
        n: usize,
        m: usize,
        es: [(Usize1, Usize1); m],
    }

    let bad = 1_000_000 as usize;

    let mut res = vec![bad; n];
    res[0] = 0;

    let mut al = AdjList::new(n);
    for e in es {
        al.add_edge(e.0, e.1);
        al.add_edge(e.1, e.0);
    }
    let mut q = VecDeque::new();
    q.push_back(0 as usize);
    while !q.is_empty() {
        let now = q.pop_front().unwrap();
        let nxts = al.out_edges(now);
        for nxt in nxts {
            if res[nxt] == bad {
                res[nxt] = now;
                q.push_back(nxt);
            }
        }
    }

    if res.contains(&bad) {
        println!("No");
    } else {
        println!("Yes");
        for i in 1..n {
            println!("{}", res[i] + 1);
        }
    }
}
