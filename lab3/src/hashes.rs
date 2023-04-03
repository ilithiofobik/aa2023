use digest::Digest;
use blake2::Blake2s256; // 256 bits
use sha1::Sha1; // 160 bits 

const TWO_POW_32 : f64 = 4294967296.0;

fn bytes_to_u32(slice: &[u8]) -> u32 {
    let mut sum = 0;
    for &byte in slice.iter().take(4) {
        sum *= 256;
        sum += byte as u32;
    }
    sum
}

pub fn blake2_hash_u32(n: u32) -> u32 {
    let mut hasher = Blake2s256::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    bytes_to_u32(slice)
}

pub fn sha1_hash_u32(n: u32) -> u32 {
    let mut hasher = Sha1::new();
    hasher.update(n.to_be_bytes());
    let res = hasher.finalize();
    let slice = res.as_slice();
    bytes_to_u32(slice)
}

pub fn my_hash_u32(n: u32) -> u32 {
    let ones32 = std::u32::MAX;
    let shift = n % 32;
    let n = n.rotate_left(shift);

    match shift % 2 {
        0 => n ^ ones32,
        1 => n,
        _ => unreachable!(),
    }
}

pub fn blake2_hash_f64(n: u32) -> f64 {
    blake2_hash_u32(n) as f64 / TWO_POW_32
}