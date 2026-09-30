pub fn square(s: u32) -> u64 {
    (2u64).pow(s - 1)
}

pub fn total() -> u64 {
    (0u64..64u64).into_iter().map(|p| 2u64.pow(p as u32)).sum()
}
