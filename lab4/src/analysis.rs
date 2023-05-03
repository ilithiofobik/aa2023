pub fn nakamoto(n: u64, q: f64) -> f64 {
    let p = 1.0 - q;
    let mut sum = 0.0;
    let mut k_fact = 1u64;
    let qp = q / p;
    let mut q_n_k = qp;
    let lambda = (n as f64 * q) / p;
    let mut lam_k = lambda;
    for k in 0..n {
        sum += lam_k * (1.0 - q_n_k) / k_fact as f64;
        k_fact *= k + 1;
        q_n_k *= qp;
        lam_k *= lambda;
    }
    1.0 - sum * (-lambda).exp()
}

pub fn grunspan(n: u64, q: f64) -> f64 {
    let p = 1.0 - q;
    let mut p_n_q_k = p.powi(n as i32);
    let mut q_n_p_k = q.powi(n as i32);
    let mut binomial = 1u64;
    let mut sum = 0.0;
    for k in 0..n {
        sum += binomial as f64 * (p_n_q_k - q_n_p_k);
        binomial *=  n + k;
        binomial /= k + 1;
        p_n_q_k *= q;
        q_n_p_k *= p;
    }
    1.0 - sum
}