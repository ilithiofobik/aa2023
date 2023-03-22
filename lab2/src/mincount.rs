use super::multiset::MultiSet;

pub fn mincount(m: MultiSet, h: fn(usize, usize) -> f64, k: usize, b: usize) -> f32 {
    let mut m_arr = vec![1.0; k];
    for x in m {
        let hx = h(x, b);
        if hx < m_arr[k - 1] && !m_arr.contains(&hx) {
            m_arr[k - 1] = hx;
            m_arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        } 
    }
    
    match m_arr[k - 1] == 1.0 {
        true  => m_arr.into_iter().filter(|&x| x != 1.0).count() as f32,
        false => (k as f32 - 1.0) / m_arr[k - 1] as f32
    }
}