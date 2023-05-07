mod experiments;
mod analysis;
mod simulation;

const MAX_ITER: u64 = 100_000; // maximal iteration in one monte carlo simulation
const NUM_OF_RUNS: usize = 10_000; // number of monte carlo simulations

fn main() {
    let ns = [1, 3, 6, 12, 24, 48];
    let ppbs = [0.001, 0.01, 0.1];
    let qs = (5..=45).map(|n| n as f64 / 100.0).collect::<Vec<f64>>();

    experiments::experiment9a(&ns, &qs, &ppbs, NUM_OF_RUNS, MAX_ITER);
}
