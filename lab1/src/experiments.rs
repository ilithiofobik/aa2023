use std::collections::HashMap;
use super::leader;

pub fn exact_experiment(n: usize, num_of_exps: usize) -> Vec<(usize, usize)> {
    let mut map = HashMap::new();
    for _ in 0..num_of_exps {
        let leader = leader::leader_exact(n);
        let count = map.get(&leader).unwrap_or(&0) + 1;
        map.insert(leader, count);
    }
    
    let mut result: Vec<(usize, usize)> = map.into_iter().collect();
    result.sort();
    result
}

pub fn bound_experiment(n: usize, u: usize, num_of_exps: usize) -> Vec<(usize, usize)> {
    let mut map = HashMap::new();
    for _ in 0..num_of_exps {
        let leader = leader::leader_bound(n, u);
        let count = map.get(&leader).unwrap_or(&0) + 1;
        map.insert(leader, count);
    }
    
    let mut result: Vec<(usize, usize)> = map.into_iter().collect();
    result.sort();
    result
}

pub fn round_experiment(n: usize, l: usize, num_of_exps: usize) -> f64 {
    let mut counter = 0;
    for _ in 0..num_of_exps {
        counter += leader::leader_round(n, l);
    }
    (counter as f64) / (num_of_exps as f64)
}