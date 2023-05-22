use super::graphs::Graph;
use super::graphs::Color;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use std::thread::JoinHandle;

#[allow(clippy::needless_collect)]
pub fn maximal_independent(g: &Graph) -> HashSet<usize> {
    let n = g.n;
    let independent = Arc::new(Mutex::new(HashSet::new()));
    let graph = Arc::new(Mutex::new((*g).clone()));

    let processes: Vec<JoinHandle<()>> = 
        (0..n)
        .map(|i| {
            let graph = Arc::clone(&graph);
            let independent = Arc::clone(&independent);

            thread::spawn(move || {
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
            })
        }).collect();
    
    processes.into_iter().for_each(|p| p.join().unwrap());
   
    let lock = Arc::try_unwrap(independent).expect("Lock still has multiple owners");
    lock.into_inner().expect("Mutex cannot be locked")
}