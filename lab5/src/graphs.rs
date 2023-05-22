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
pub struct Graph {
    pub n: usize,
    pub colors: Vec<Color>,
    pub neighbours: Vec<HashSet<usize>>,
}

impl Graph {
    fn add_edge(&mut self, u: usize, v: usize) {
        self.neighbours[u].insert(v);
        self.neighbours[v].insert(u);
    }

    fn new(n: usize) -> Self {
        Self { n, colors: vec![Color::Yellow; n], neighbours: vec![HashSet::new(); n] }
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

        g
    }

    fn new_color(neighbours: &HashSet<usize>, independent: &HashSet<usize>, i: usize) -> Color {
        let is_independent = independent.contains(&i);
        let has_neighbours_in_independent = neighbours.iter().any(|v| independent.contains(v));

        match (is_independent, has_neighbours_in_independent) {
            (true, true)   => Color::Red,
            (false, false) => Color::Yellow,
            (false, true)  => Color::White,
            (true, false)  => Color::Black,
        }
    }

    pub fn update_color(&mut self, independent: &HashSet<usize>, i: usize) {
        self.colors[i] = Graph::new_color(&self.neighbours[i], independent, i);
    }

    pub fn update_color_with_neighs(&mut self, independent: &HashSet<usize>, i: usize) {
        self.update_color(independent, i);

        for &j in self.neighbours[i].iter() {
            self.colors[j] = Graph::new_color(&self.neighbours[j], independent, j);
        }
    }
}

