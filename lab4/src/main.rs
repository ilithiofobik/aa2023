mod experiments;
mod analysis;
mod simulation;

const MAX_ITER: usize = 1_000; // maximal iteration in one monte carlo simulation
const NUM_OF_RUNS: usize = 1_000; // number of monte carlo simulations

fn main() {
    let ns = [1, 3, 6, 12, 24, 48];
    let ppbs = [0.001, 0.01, 0.1];

    experiments::experiment9a(&ns, &ppbs, NUM_OF_RUNS, MAX_ITER);
}
