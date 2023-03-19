pub fn simple_multiset(n: usize) -> Vec<(usize, usize)> {
    let first = (n * (n - 1)) / 2;

    (first..first + n).map(|x| (x, 1)).collect()
}