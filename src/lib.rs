pub mod bit;
pub mod bloom;

// m represent capacity of set, h represent hash length
pub struct Bloom {
    m: u32,
    h: u32,
    bit_set: BitSet,
}

pub struct BitSet {
    pub len: u32,
    pub sets: Vec<u64>,
}

pub fn new_bloom_filter(m: u32, k: u32) -> Option<Bloom> {
    Bloom::new_bloom(m, k)
}
