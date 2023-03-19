use digest::Digest;
use blake2::Blake2b512;
use sha3::Sha3_256;

fn bytes_to_f64(slice: &[u8]) -> f64 {
    let mut sum = 0.0;
    for &byte in slice {
        sum += byte as f64;
        sum /= 256.0;
    }
    sum
}

pub fn blake2_hash(n: usize) -> f64 {
    let mut hasher = Blake2b512::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    bytes_to_f64(slice)
}

pub fn _sha3_hash(n: usize) -> f64 {
    let mut hasher = Sha3_256::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    bytes_to_f64(slice)
}