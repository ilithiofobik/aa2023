pub fn nakamoto(n: u64, q: f64) -> f64 {
    let p = 1.0 - q;
    let mut sum = 0.0;
    let mut k_fact = 1.0;
    let qp = q / p;
    let mut q_n_k = qp;
    let lambda = (n as f64 * q) / p;
    let mut lam_k = lambda;
    for k in 0..n {
        sum += lam_k * (1.0 - q_n_k) / k_fact;
        k_fact *= (k + 1) as f64;
        q_n_k *= qp;
        lam_k *= lambda;
    }
    1.0 - sum * (-lambda).exp()
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