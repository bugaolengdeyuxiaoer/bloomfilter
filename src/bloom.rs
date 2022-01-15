pub use std::ops::{BitAnd, Shr};
pub use crate::BitSet;
pub use crate::Bloom;

const _128_LOW: u128 = u128::MAX & u64::MAX as u128;
const _128_HIGH: u128 = u128::MAX & !u64::MAX as u128;

impl Bloom {
    pub fn new_bloom(m: u32, h: u32) -> Option<Bloom> {
        let r = BitSet::new_bit_set(m);
        Some(Bloom { m, h, bit_set: r })
    }

    // Cap returns the capacity, _m_, of a Bloom filter
    pub fn cap(&self) -> u32 {
        return self.m;
    }

    // K returns the number of hash functions used in the BloomFilter
    pub fn k(&self) -> u32 {
        return self.h;
    }
    pub fn add(&mut self, data: &[u8]) {
        let h = self.base_hashes(data);
        for i in 0..self.h {
            let j = self.Location(h, i);
            self.bit_set.set(j)
        }
    }
    pub fn is_set(&mut self, data: &[u8]) -> bool {
        let h = self.base_hashes(data);
        for i in 0..self.h {
            if !self.bit_set.is_set(i) {
                return false;
            }
        }
        true
    }
    // base_hashes returns the four hash values of data that are used to create k hashes
    fn base_hashes(&mut self, mut data: &[u8]) -> [u64; 4] {
        let mut a1: &[u8] = &[1];
        let hasher = match murmur3::murmur3_x64_128(&mut data, 0) {
            Err(e) => return [0; 4],
            Ok(h) => h,
        };
        let v1: u64 = hasher.bitand(_128_LOW) as u64;
        let v2: u64 = hasher.bitand(_128_HIGH).shr(64) as u64;
        let hasher = match murmur3::murmur3_x64_128(&mut a1, 0) {
            Err(e) => return [0; 4],
            Ok(h) => h,
        };
        let v3: u64 = hasher.bitand(_128_LOW) as u64;
        let v4: u64 = hasher.bitand(_128_HIGH).shr(64) as u64;
        [v1, v2, v3, v4]
    }

    fn Location(&mut self, h: [u64; 4], i: u32) -> u32 {
        return (self.location(h, i) % self.m as u64) as u32;
    }

    // left two bit of i
    // 00 -> h[0] + i * h[2]
    // 10 -> h[0] + i * h[3]
    // 01 -> h[1] + i * h[2]
    // 11 -> h[1] + i * h[3]
    fn location(&mut self, h: [u64; 4], i: u32) -> u64 {
        let i_64 = i as u64;
        return h[i_64.bitand(1) as usize]
            + i_64 * h[2 + ((i_64 + i_64.bitand(1)).bitand(3)).shr(1) as usize];
    }

    fn clear(&mut self) {
        for i in 0..self.m {
            self.bit_set.clear(i)
        }
    }
}
