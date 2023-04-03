use super::multiset::MultiSet;

fn alpha(m: u32) -> f64 {
    match m {
        16 => 0.673,
        32 => 0.697,
        64 => 0.709,
        m => 0.7213 / (1.0 + 1.079 / (m as f64))
    }
}

fn rho(x: u32) -> u32 {
    x.leading_zeros() + 1
}

pub fn hyper_log_log(multiset: MultiSet, h: fn(u32) -> u32, b: usize) -> f64 {
    let m = 2usize.pow(b as u32);
    let m_f64 = m as f64;
    let mut m_arr = vec![0; m];
    let two_pow_32 = 2.0f64.powf(32.0);
    let alpha = alpha(m as u32);

    for v in multiset {
        let x = h(v);
        let (j, w) = (x >> (32 - b), x << b);
        m_arr[j as usize] = m_arr[j as usize].max(rho(w));
    }

    let powers_sum = m_arr.iter().map(|&x| 2.0f64.powi(-(x as i32))).sum::<f64>();
    let e = (alpha * m_f64 * m_f64) / powers_sum;
    
    if e <= (5.0 * m_f64) / 2.0 {
        let v = m_arr.iter().filter(|&&x| x == 0).count();
        if v != 0 {
            m_f64 * (m_f64 / v as f64).ln()
        } else {
            e
        }
    } else if e > two_pow_32 / 30.0 {
        -two_pow_32 * (1.0 - (e / two_pow_32)).ln()
    } else {
        e
    }
}