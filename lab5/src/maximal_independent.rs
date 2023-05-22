use super::graphs::Graph;
use super::graphs::Color;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

pub fn maximal_independent(g: &Graph) -> HashSet<usize> {
    let n = g.n;
    let independent = Arc::new(Mutex::new(HashSet::new()));
    let graph = Arc::new(Mutex::new((*g).clone()));
    let mut processes = vec![];

    for i in 0..n {
        let graph = Arc::clone(&graph);
        let independent = Arc::clone(&independent);

        let process = thread::spawn(move || {
            loop {
                let time = time::Duration::from_millis(fastrand::u64(10..100));
                thread::sleep(time);

                let mut independent = independent.lock().unwrap();
                let mut graph = graph.lock().unwrap();

                match graph.colors[i] {
                    Color::Red => {
                        independent.remove(&i);
                        graph.update_color_with_neighs(&independent, i);
                    },
                    Color::Yellow => {
                        independent.insert(i);
                        graph.update_color(&independent, i);
                    },
                    _ => (),
                }

                if graph.colors.iter().all(|&color| color == Color::Black || color == Color::White) {
                    break;
                }
            }
        });
        processes.push(process);
    }

    for p in processes {
        p.join().unwrap();
    }

    let mut result = HashSet::new();
    for &v in independent.lock().unwrap().iter() {
        result.insert(v);
    }
    result
}