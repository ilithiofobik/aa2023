mod mincount;
mod multiset;
mod hashes;
mod experiments;

fn main() {
    let ns: [usize; 10_000] = (1..=10_000).collect::<Vec<usize>>().try_into().unwrap();
    // note: https://www.wolframalpha.com/input?i=%281-10000%2F2%5E%2848%29%29%5E10000
    //experiments::experiment5b(&ns);
    //experiments::experiment5c(&ns, 9500); // Result = 333
    //experiments::experiment6(&ns);

    for i in (1..5) {
        let m = multiset::MultiSet::new_with_multiplier(i, 1);
        println!("Start with {}", i);
        for x in m {
            println!("{}", x);
        }
        println!("Done with {}", i);
    }
}
