#[allow(dead_code)]
fn chernoff(delta: f64, k: f64) -> f64 {
    let eps = delta / (1.0 + delta);
    let eta = delta / (1.0 - delta);
    let f_k_eps = (eps * k).exp() * (1.0 - eps).powf(k);
    let f_k_eta = (-eta * k).exp() * (1.0 + eta).powf(k);
    f_k_eps + f_k_eta
}

#[allow(dead_code)]
pub fn chernoff_delta(alpha: f64, k: f64) -> f64 {
    let mut left = 0.0;
    let mut right = 1.0;
    while (right - left) > 0.000000001 {
        let mid = (left + right) / 2.0;
        let chernoff = chernoff(mid, k);
        if chernoff > alpha {
            left = mid;
        } else {
            right = mid;
        }
    }
    (right + left) / 2.0
}

#[allow(dead_code)]
pub fn chebyshev_delta(alpha: f64, k: f64) -> f64 {
    (1.0 / (k * alpha)).sqrt()
} 

#[allow(dead_code)]
pub fn simulation_delta(alpha: f64, exp: &Vec<(f64, f64)>) -> f64 {
    let n = exp.len();
    let min_success = ((1.0 - alpha) * n as f64) as usize;
    let mut left = 0.0;
    let mut right = 1.0;
    while (right - left) > 0.000000001 {
        let mid = (left + right) / 2.0;
        let success = exp.iter().map(|&(_, x)| if (x - 1.0).abs() < mid { 1 } else { 0 }).sum::<usize>();
        if success >= min_success {
            right = mid;
        } else {
            left = mid;
        }
    }
    left
} 

