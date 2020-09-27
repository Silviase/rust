use std::cmp::Ordering;
use std::collections::BinaryHeap;
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]

pub struct Edge {
    to: usize,
    cost: usize,
}

impl Edge {
    fn new(to: usize, cost: usize) -> Edge {
        Edge { to, cost }
    }
}

pub struct AdjList {
    dim: usize,
    list: Vec<Vec<Edge>>,
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
        self.list.get_mut(from).unwrap().push(Edge::new(to, 0));
    }

    pub fn add_undirected_edge(&mut self, from: usize, to: usize) {
        self.list.get_mut(from).unwrap().push(Edge::new(to, 0));
        self.list.get_mut(to).unwrap().push(Edge::new(from, 0));
    }

    pub fn out_edges(&mut self, vertex: usize) -> Vec<Edge> {
        self.list.get_mut(vertex).unwrap().to_vec()
    }

    pub fn in_edges(&mut self, v: usize) -> Vec<usize> {
        let mut res = vec![];
        for l in 0..self.dim() {
            if let Some(list) = self.list.get(l) {
                if list.contains(&Edge::new(v, 0)) {
                    res.push(l);
                }
            }
        }

        res
    }

    fn shortest_path(&self, start: usize, goal: usize) -> Option<usize> {
        let mut dist: Vec<_> = (0..self.list.len()).map(|_| usize::MAX).collect();
        let mut heap = BinaryHeap::new();
        dist[start] = 0;
        heap.push(State {
            cost: 0,
            pos: start,
        });
        while let Some(State { cost, pos }) = heap.pop() {
            if pos == goal {
                return Some(cost);
            }
            if cost > dist[pos] {
                continue;
            }
            for edge in &self.list[pos] {
                let next = State {
                    cost: cost + edge.cost,
                    pos: edge.to,
                };
                if next.cost < dist[next.pos] {
                    heap.push(next);
                    dist[next.pos] = next.cost;
                }
            }
        }
        None
    }

    fn shortest_paths(&self, start: usize) -> Vec<usize> {
        let mut dist: Vec<_> = (0..self.list.len()).map(|_| usize::MAX).collect();
        let mut heap = BinaryHeap::new();
        dist[start] = 0;
        heap.push(State {
            cost: 0,
            pos: start,
        });
        while let Some(State { cost, pos }) = heap.pop() {
            if cost > dist[pos] {
                continue;
            }
            for edge in &self.list[pos] {
                let next = State {
                    cost: cost + edge.cost,
                    pos: edge.to,
                };
                if next.cost < dist[next.pos] {
                    heap.push(next);
                    dist[next.pos] = next.cost;
                }
            }
        }
        dist
    }
}
