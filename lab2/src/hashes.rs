use digest::Digest;
use blake2::Blake2s256; // 256 bits
use sha1::Sha1; // 160 bits 

fn bytes_to_f64(slice: &[u8], num_of_bytes: usize) -> f64 {
    let mut sum = 0.0;
    for &byte in slice.iter().take(num_of_bytes) {
        sum += byte as f64;
        sum /= 256.0;
    }
    sum
}

pub fn blake2_hash(n: usize, num_of_bytes: usize) -> f64 {
    let mut hasher = Blake2s256::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    bytes_to_f64(slice, num_of_bytes)
}

pub fn sha1_hash(n: usize, num_of_bytes: usize) -> f64 {
    let mut hasher = Sha1::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    bytes_to_f64(slice, num_of_bytes)
}

pub fn my_hash(n: usize, num_of_bytes: usize) -> f64 {
    let mut result = n;
    let max_val = 2usize.pow(8 * (num_of_bytes as u32));
    for _ in 0..num_of_bytes {
        result = (result * result) % max_val;
    }
    result as f64 / max_val as f64
}