use super::mincount;
use super::hashes;
use super::multiset;

use std::fs::File;
use std::io::Write;

fn to_file_exp(filename: &str, k: usize, hash: fn(usize, usize) -> f64, ns: &[usize], b: usize) {
    let mut f = File::create(filename).unwrap();
    for &n in ns {
        let m = multiset::MultiSet::new(n);
        let count = mincount::mincount(m, hash, k, 6);
        let n = n as f32;
        writeln!(f, "{};{}", n, count as f32 / n).unwrap();
    }
}

fn k_experiment(k: usize, hash: fn(usize, usize) -> f64, ns: &[usize], b: usize) -> Vec<(f32, f32)> {
    ns 
    .iter()
    .map(|&n| {
        let m = multiset::MultiSet::new(n);
        let count = mincount::mincount(m, hash, k, b);
        let n = n as f32;
        (n, count as f32 / n)
    })
    .collect::<Vec<(f32, f32)>>()
}

fn avg_diff_experiment(k: usize, hash: fn(usize, usize) -> f64, ns: &[usize], b: usize) -> f32 {
    let results = k_experiment(k, hash, ns, b);
    let sum = results.iter().map(|(_, diff)| (diff - 1.0).abs()).sum::<f32>();
    sum / (ns.len() as f32)
}

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

// pub fn experiment6(ns: &[usize]) {
//     println!("Running experiment 6...");

//     for k in ks {
//         let filename = format!("data\\exp5b_k_{}.csv", k);
//         let mut f = File::create(filename).unwrap();
//         for &n in ns {
//             let m = multiset::MultiSet::new(n);
//             let count = mincount::mincount(m, hashes::blake2_hash, k, 6);
//             let n = n as f32;
//             writeln!(f, "{};{}", n, count as f32 / n).unwrap();
//         }
//         println!("Done running experiment for k = {}.", k)
//     }

//     let bytes_arr = [1, 2, 3, 4, 5, 6];
//     for byte in bytes_arr {
//         println!("Running experiment for b = {} bits...", 8 * byte);
//         let sha1_avg_diff = avg_diff_experiment(400, hashes::sha1_hash, ns, byte);
//         println!("sha1_avg_diff = {}", sha1_avg_diff);
//         let blake2_avg_diff = avg_diff_experiment(400, hashes::blake2_hash, ns, byte);
//         println!("blake2_avg_diff = {}", blake2_avg_diff);
//     }

//     println!("Done running experiment 6.")
// }