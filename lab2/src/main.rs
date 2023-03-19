mod mincount;
mod multiset;
mod hashes;
mod experiments;
//mod plots;

fn main() {
    let ks = vec![2,4,8,16,32];
    let ns = (1..=10_000).collect::<Vec<usize>>();
    let results = experiments::quotient_experiment(&ks, &ns);
    let ns_f32 = ns.iter().map(|&x| x as f32).collect::<Vec<f32>>();
    //plots::plot_quotient_experiment(results, &ns_f32);
}
