pub fn log_floor(x: u32) -> u32 {
    return u32::BITS - x.leading_zeros() - 1;
}

pub fn max(a: usize, b: usize) -> usize {
    match a < b {
        true => b,
        false => a,
    }
}