use super::mincount;
use super::hashes;
use super::multiset;
use super::bounds_calc;

use std::fs::File;
use std::io::Write;

fn to_file_exp(filename: &str, k: usize, hash: fn(usize, usize) -> f64, ns: &[usize], b: usize) {
    let mut f = File::create(filename).unwrap();
    for &n in ns {
        let m = multiset::MultiSet::new(n);
        let count = mincount::mincount(m, hash, k, b);
        let n = n as f32;
        writeln!(f, "{};{}", n, count as f32 / n).unwrap();
    }
}

#[allow(dead_code)]
fn k_experiment(k: usize, hash: fn(usize, usize) -> f64, ns: &[usize], b: usize) -> Vec<(f64, f64)> {
    ns 
    .iter()
    .map(|&n| {
        let m = multiset::MultiSet::new(n);
        let count = mincount::mincount(m, hash, k, b);
        let n = n as f64;
        (n, count as f64 / n)
    })
    .collect::<Vec<(f64, f64)>>()
}

#[allow(dead_code)]
pub fn experiment5a() {
    println!("Running experiment 5a...");

    let multi_single = multiset::MultiSet::new(10000);
    let multi_double = multiset::MultiSet::new_with_multiplier(10000, 2);
    
    let single_mincount = mincount::mincount(multi_single, hashes::blake2_hash, 400, 6);
    let double_mincount = mincount::mincount(multi_double, hashes::blake2_hash, 400, 6);

    if single_mincount == double_mincount {
        println!("Mincount for single and double multiset is equal.");
    } else {
        println!("Mincount for single and double multiset is not equal.");
    }

    println!("Done running experiment 5a.")
}

#[allow(dead_code)]
pub fn experiment5b(ns: &[usize]) {
    println!("Running experiment 5b...");

    let ks = [2, 3, 10, 100, 400];

    for k in ks {
        let filename = format!("data\\exp5b_k_{}.csv", k);
        to_file_exp(&filename, k, hashes::blake2_hash, ns, 6);
        println!("Done running experiment for k = {}.", k)
    }

    println!("Done running experiment 5b.")
}

#[allow(dead_code)]
pub fn experiment5c(ns: &[usize], min_count: usize) {
    println!("Running experiment 5c...");

    let mut left = 2;
    let mut right = 400;

    while left < right {
        let mid = (left + right) / 2;
        println!("running experiment for k = {}... ", mid);
        let arr = k_experiment(mid, hashes::blake2_hash, ns, 6);
        let good: usize = arr.iter().map(|&(_, x)| if x > 0.9 && x < 1.1 { 1 } else { 0 }).sum();
        if good >= min_count {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    println!("k = {}", left);
    println!("Done running experiment 5c.")
}

#[allow(dead_code)]
pub fn experiment6(ns: &[usize]) {
    println!("Running experiment 6...");

    let bs = [1,2,4,6];
    let hashes = [hashes::sha1_hash, hashes::blake2_hash, hashes::my_hash];
    let hashes_names = ["sha1", "blake2", "my_hash"];

    for (&hash, hash_name) in hashes.iter().zip(hashes_names.iter()) {
        for &b in bs.iter() {
            println!("Running experiment for b = {} bits for hash {}...", 8 * b, hash_name);
            let filename = "data\\exp6_".to_string() + hash_name + &format!("_b_{}.csv", 8 * b);
            to_file_exp(&filename, 400, hash, ns, b); 
        }
    }

    println!("Done running experiment 6.")
}


#[allow(dead_code)]
pub fn experiment7(ns: &[usize]) {
    println!("Running experiment 7...");

    let exp = k_experiment(400, hashes::blake2_hash, ns, 6);

    for alpha in [0.05, 0.01, 0.005] {
        let chernoff_bound = bounds_calc::chernoff_delta(alpha, 400.0);
        let chebyshev_bound = bounds_calc::chebyshev_delta(alpha, 400.0);
        let simulation_bound = bounds_calc::simulation_delta(alpha, &exp);
        
        for (title, bound) in [("chernoff", chernoff_bound), ("chebyshev", chebyshev_bound), ("simulation", simulation_bound)] {
            let filename = "data\\exp7_".to_string() + title + &format!("_alpha_{}.txt", alpha);
            let mut f = File::create(filename).unwrap();
            writeln!(f, "{}", bound).unwrap();
        }
      
        println!("alpha = {}, chernoff={}", alpha, chebyshev_bound);
        println!("alpha = {}, chebyshev={}", alpha, chernoff_bound);
        println!("alpha = {}, simulation={}", alpha, simulation_bound);
    }

    println!("Done running experiment 7.")
}