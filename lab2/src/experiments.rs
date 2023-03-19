use super::mincount;
use super::hashes;
use super::multiset;

pub fn k_experiment(k: usize, ns: &[usize]) -> Vec<(f32, f32)> {
    ns 
    .iter()
    .map(|&n| {
        let m = multiset::simple_multiset(n);
        let count = mincount::mincount(&m, hashes::blake2_hash, k);
        let n = n as f32;
        (n, count as f32 / n)
    })
    .collect::<Vec<(f32, f32)>>()
}

pub fn quotient_experiment(ks: &[usize], ns: &[usize]) -> Vec<(usize, Vec<(f32, f32)>)> {
    ks
    .iter()
    .map(|&k| {
        (k, k_experiment(k, ns))
    }
    )
    .collect::<Vec<(usize, Vec<(f32, f32)>)>>()
}