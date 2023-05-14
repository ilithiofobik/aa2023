mod mutual_exclusion;

const N: usize = mutual_exclusion::NUSIZE;

fn main() {
    let timer = std::time::Instant::now();
    let max_steps = mutual_exclusion::max_steps();
    println!("max_steps for n = {}: {}", N, max_steps);
    println!("time elapsed: {:?}", timer.elapsed());
}