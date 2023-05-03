/// Returns true iff adversary wins
pub fn simulation(n: u64, q: f64, max_iter: usize) -> bool {
    let rand = fastrand::Rng::new();
    let mut adversary = 0;
    let mut legitimate = 0;

    while legitimate < n {
        if rand.f64() <= q {
            adversary += 1;
        } else {
            legitimate += 1;
        }
    }

    for _ in 0..max_iter {
        if adversary >= legitimate {
            return true
        }
        if rand.f64() <= q {
            adversary += 1;
        } else {
            legitimate += 1;
        }
    }

    false
}