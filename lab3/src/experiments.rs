use super::mincount;
use super::hashes;
use super::multiset;
use super::hyperloglog;

use std::fs::File;
use std::io::Write;

fn to_file_exp_mc(filename: &str, k: usize, hash: fn(u32) -> f64, ns: &[u32]) {
    let mut f = File::create(filename).unwrap();
    for &n in ns {
        let m = multiset::MultiSet::new(n);
        let count = mincount::mincount(m, hash, k);
        let n = n as f32;
        writeln!(f, "{};{}", n, count as f32 / n).unwrap();
    }
}

fn to_file_exp_hll(filename: &str, hash: fn(u32) -> u32, ns: &[u32], b: usize) {
    let mut f = File::create(filename).unwrap();
    for &n in ns {
        let m = multiset::MultiSet::new(n);
        let count = hyperloglog::hyper_log_log(m, hash, b);
        let n = n as f64;
        writeln!(f, "{};{}", n, count / n).unwrap();
    }
}

#[allow(dead_code)]
pub fn experiment8a(ns: &[u32]) {
    println!("Running experiment 8a...");

    let bs = [4,5,6,7,12];
    let hashes = [hashes::sha1_hash_u32, hashes::blake2_hash_u32, hashes::my_hash_u32];
    let hashes_names = ["sha1", "blake2", "my_hash"];

    for (&hash, hash_name) in hashes.iter().zip(hashes_names.iter()) {
        for b in bs {
            let filename = "data\\exp8a_".to_string() + hash_name + &format!("_b_{}.csv", b);
            to_file_exp_hll(&filename, hash, ns, b);
            println!("Done running experiment for b = {}.", b);
        }
    }

    println!("Done running experiment 8a.")
}

#[allow(dead_code)]
pub fn experiment8b(ns: &[u32]) {
    println!("Running experiment 8b...");

    let ks = [5,10,20,640];
    let hashes = [hashes::blake2_hash_f64];
    let hashes_names = ["blake2"];

    for (&hash, hash_name) in hashes.iter().zip(hashes_names.iter()) {
        for k in ks {
            let filename = "data\\exp8b_".to_string() + hash_name + &format!("_k_{}.csv", k);
            to_file_exp_mc(&filename, k, hash, ns);
            println!("Done running experiment for k = {}.", k);
        }
    }

    println!("Done running experiment 8b.")
}

// #[allow(dead_code)]
// pub fn experiment7(ns: &[usize]) {
//     println!("Running experiment 7...");

//     let exp = k_experiment(400, hashes::blake2_hash, ns, 6);

//     for alpha in [0.05, 0.01, 0.005] {
//         let chernoff_bound = bounds_calc::chernoff_delta(alpha, 400.0);
//         let chebyshev_bound = bounds_calc::chebyshev_delta(alpha, 400.0);
//         let simulation_bound = bounds_calc::simulation_delta(alpha, &exp);
        
//         for (title, bound) in [("chernoff", chernoff_bound), ("chebyshev", chebyshev_bound), ("simulation", simulation_bound)] {
//             let filename = "data\\exp7_".to_string() + title + &format!("_alpha_{}.txt", alpha);
//             let mut f = File::create(filename).unwrap();
//             writeln!(f, "{}", bound).unwrap();
//         }
      
//         println!("alpha = {}, chernoff={}", alpha, chebyshev_bound);
//         println!("alpha = {}, chebyshev={}", alpha, chernoff_bound);
//         println!("alpha = {}, simulation={}", alpha, simulation_bound);
//     }

//     println!("Done running experiment 7.")
// }