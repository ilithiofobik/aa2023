use super::analysis;
use super::simulation;

use std::fs::File;
use std::io::Write;

fn find_naka_n(q: f64, ppb: f64) -> u64 {
    let mut right = 1;

    while analysis::nakamoto(right, q) >= ppb {
        right *= 2;
    }
    let mut left = right / 2;

    while left < right {
        let mid = (left + right) / 2;
        if analysis::nakamoto(mid, q) >= ppb {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    right
}

fn find_grun_n(q: f64, ppb: f64) -> u64 {
    let mut right = 1;

    while analysis::grunspan(right, q) >= ppb {
        right *= 2;
    }
    let mut left = right / 2;

    while left < right {
        let mid = (left + right) / 2;
        if analysis::grunspan(mid, q) >= ppb {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    right
}

fn experiment_a1(ns: &[u64], qs: &[f64], num_of_runs: usize, max_iter: usize) {
    for &n in ns  {
        println!("Running experiment to find P(n) for n = {}...", n);
        let g_filename = format!("data\\grunspan_ppb_n_{}.csv", n);
        let n_filename = format!("data\\nakamoto_ppb_n_{}.csv", n);
        let s_filename = format!("data\\simulation_ppb_n_{}.csv", n);

        let mut g_file = File::create(&g_filename).unwrap();
        let mut n_file = File::create(&n_filename).unwrap();
        let mut s_file = File::create(&s_filename).unwrap();

        for &q in qs {
            let grun = analysis::grunspan(n, q);
            let naka = analysis::nakamoto(n, q);
            let mut sim = 0.0;
            for _ in 0..num_of_runs {
                if simulation::simulation(n, q, max_iter) {
                    sim += 1.0;
                }
            }
            sim /= num_of_runs as f64;
            writeln!(g_file, "{};{}", q, grun).unwrap();
            writeln!(n_file, "{};{}", q, naka).unwrap();
            writeln!(s_file, "{};{}", q, sim).unwrap();
        }
        println!("Done running experiment to find P(n) for n = {}.", n)
    }
}

fn experiment_a2(ppbs: &[f64], qs: &[f64]) {
    for &ppb in ppbs  {
        println!("Running experiment to find n for ppb = {}...", ppb);
        let g_filename = format!("data\\grunspan_find_n_ppb_{}.csv", ppb);
        let n_filename = format!("data\\nakamoto_find_n_ppb_{}.csv", ppb);

        let mut g_file = File::create(&g_filename).unwrap();
        let mut n_file = File::create(&n_filename).unwrap();

        for &q in qs {
            let grun = find_grun_n(q, ppb);
            let naka = find_naka_n(q, ppb);
            writeln!(g_file, "{};{}", q, grun).unwrap();
            writeln!(n_file, "{};{}", q, naka).unwrap();
        }
        println!("Done running experiment to find n for ppb = {}.", ppb)
    }
}

#[allow(dead_code)]
pub fn experiment9a(ns: &[u64], ppbs: &[f64], num_of_runs: usize, max_iter: usize) {
    println!("Running experiment 9...");

    let qs = (5..=45).map(|n| n as f64 / 100.0).collect::<Vec<f64>>();

    experiment_a1(ns, &qs, num_of_runs, max_iter);
    experiment_a2(ppbs, &qs);

    println!("Done running experiment 9.");
}
