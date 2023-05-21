use std::collections::HashSet;
use fastrand::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Red,
    Yellow,
    Black,
    White,
}

#[derive(Clone)]
pub struct Node {
    pub id : usize,
    pub color: Color,
    pub neighbors: HashSet<usize>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self { id, color: Color::Yellow, neighbors: HashSet::new() }
    }

    pub fn update_color(&mut self, independent: &HashSet<usize>) {
        let is_independent = independent.contains(&self.id);
        let has_neighbours_in_independent = self.neighbors.iter().any(|v| independent.contains(v));

        match (is_independent, has_neighbours_in_independent) {
            (true, true)   => self.color = Color::Red,
            (false, false) => self.color = Color::Yellow,
            (false, true)  => self.color = Color::White,
            (true, false)  => self.color = Color::Black,
        }
    }
}

#[derive(Clone)]
pub struct Graph {
    pub n: usize,
    pub nodes: Vec<Node>,
}

impl Graph {
    fn add_edge(&mut self, u: usize, v: usize) {
        self.nodes[u].neighbors.insert(v);
        self.nodes[v].neighbors.insert(u);
    }

    fn new(n: usize) -> Self {
        let mut g = Self { n, nodes : vec![] };

        for i in 0..n {
            g.nodes.push(Node::new(i));
        }

        g
    }

    pub fn new_rand(n: usize, p: f64) -> Self {
        let mut g = Self::new(n);
        let rng = Rng::new();

        for u in 0..n {
            for v in u + 1..n {
                if rng.f64() < p {
                    g.add_edge(u, v);
                }
            }
        }

        let independent = HashSet::new();
        for node in g.nodes.iter_mut() {
            node.update_color(&independent);
        }

        g
    }
}

