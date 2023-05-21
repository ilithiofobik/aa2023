mod mutual_exclusion;
mod graphs;
mod maximal_independent;

const N: usize = mutual_exclusion::NUSIZE;

fn main() {
    // mutual exclusion
    let timer = std::time::Instant::now();
    let max_steps = mutual_exclusion::max_steps();
    println!("max_steps for n = {}: {}", N, max_steps);
    println!("time elapsed: {:?}", timer.elapsed());

    // maximal independent set
    let timer = std::time::Instant::now();
    let g = graphs::Graph::new_rand(1000, 0.1);
    let independent = maximal_independent::maximal_independent(&g);
    println!("independent set: {:?}", independent);
    println!("independent set size: {}", independent.len());
    println!("time elapsed: {:?}", timer.elapsed());
}