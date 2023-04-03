mod mincount;
mod multiset;
mod hashes;
mod experiments;
mod hyperloglog;

fn main() {
    const MAX_N : u32 = 2u32.pow(15);
    let ns: [u32; MAX_N as usize] = (1..=MAX_N).collect::<Vec<u32>>().try_into().unwrap();

    experiments::experiment8a(&ns);
    experiments::experiment8b(&ns);
}
