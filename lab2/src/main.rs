mod mincount;
mod multiset;
mod hashes;
mod experiments;
mod bounds_calc;

fn main() {
    let ns: [usize; 10_000] = (1..=10_000).collect::<Vec<usize>>().try_into().unwrap();
    // note: https://www.wolframalpha.com/input?i=+Product%5B%281-i%2F%282%5E32%29%29%2C%7Bi%2C10000%7D%5D+
    //experiments::experiment5a();
    //experiments::experiment5b(&ns);
    //experiments::experiment5c(&ns, 9500); // Result = 333
    //experiments::experiment6(&ns);
    experiments::experiment7(&ns);
    //println!("Alfa = 0.05, k=400, chernof__delta-1 = {}\n", bounds_calc::chernoff_delta(0.05, 400.0));
    //println!("Alfa = 0.05, k=400, chernof__delta-1 = {}\n", bounds_calc::chernoff_delta(0.01, 400.0));
    //println!("Alfa = 0.05, k=400, chernof__delta-1 = {}\n", bounds_calc::chernoff_delta(0.005, 400.0));
}
