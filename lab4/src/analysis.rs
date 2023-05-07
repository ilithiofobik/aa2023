fn q_n_k(q: f64, n: u64, k: u64) -> f64 {
    (q / (1.0 - q)).powi((n-k) as i32)
}

pub fn nakamoto(n: u64, q: f64) -> f64 {
    let p = 1.0 - q;
    let mut sum = 0.0;
    let mut k_fact = 1.0;
    let lambda = (n as f64 * q) / p;
    let mut lam_k = 1.0;
    for k in 0..n {
        sum += lam_k * (1.0 - q_n_k(q, n, k)) / k_fact;
        k_fact *= (k + 1) as f64;
        lam_k *= lambda;
    }
    1.0 - (-lambda).exp() * sum
}

pub fn grunspan(n: u64, q: f64) -> f64 {
    let p = 1.0 - q;
    let mut p_n_q_k = p.powi(n as i32);
    let mut q_n_p_k = q.powi(n as i32);
    let mut binomial = 1.0;
    let mut sum = 0.0;
    for k in 0..n {
        sum += binomial * (p_n_q_k - q_n_p_k);
        binomial *=  (n + k) as f64 / (k + 1) as f64;
        p_n_q_k *= q;
        q_n_p_k *= p;
    }
    1.0 - sum
}