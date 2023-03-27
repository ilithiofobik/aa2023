fn chernoff(delta: f64, k: f64) -> f64 {
    let eps = delta / (1.0 + delta);
    let eta = delta / (1.0 - delta);
    let f_k_eps = (eps * k).exp() * (1.0 - eps).powf(k);
    let f_k_eta = (-eta * k).exp() * (1.0 + eta).powf(k);
    f_k_eps + f_k_eta
}

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

pub fn chebyshev_delta(alpha: f64, k: f64) -> f64 {
    (1.0 / (k * alpha)).sqrt()
} 