use std::fs::copy;
use std::ops::BitXorAssign;
use crate::BitSet;

const wordSize: u32 = 64;

const log2WordSize: u32 = 6;

const allBits: u32 = 0xffffffff;

#[derive(Debug, PartialEq)]
enum BitErr {
    // Incorrect length of slice
    BadLen,
}

impl BitSet {
    pub fn new_bit_set(len: u32) -> BitSet {
        let mut b = BitSet { len, sets: vec![] };
        b.sets = Vec::with_capacity(b.words_needed(len) as usize);
        b
    }
    fn extend_set_maybe(&mut self, i: u32) {
        if i >= self.len {
            let nsize = self.words_needed(i + 1) as usize;
            if self.sets.is_empty() {
                self.sets = Vec::with_capacity(nsize)
            } else if self.sets.capacity() >= nsize {
                self.sets.truncate(nsize)
            } else if self.sets.len() < nsize {
                let mut new_vet: Vec<u64> = Vec::with_capacity(nsize);
                new_vet.copy_from_slice(&self.sets);
                self.sets = new_vet
            }
            self.len = i + 1
        }
    }
    fn words_needed(&mut self, i: u32) -> u32 {
        if i > (self.cap() - wordSize + 1) {
            return self.cap() >> log2WordSize;
        }
        return (i + (wordSize - 1)) >> log2WordSize;
    }

    fn cap(&mut self) -> u32 {
        u32::MAX
    }
    pub fn set(&mut self, i: u32) {
        self.extend_set_maybe(i);
        self.sets[(i >> log2WordSize) as usize] |= 1 << (i & (wordSize - 1));
    }
    pub fn set_to(&mut self, i: u32, value: bool) {
        if value {
            return self.set(i);
        }
        self.clear(i)
    }
    pub fn clear(&mut self, i: u32) {
        self.sets[(i >> log2WordSize) as usize] &= !(1 << (i & (wordSize - 1)));
    }

    pub fn is_set(&self, i: u32) -> bool {
        if i >= self.len {
            return false;
        }
        self.sets[(i >> log2WordSize) as usize] & (1 << (i & (wordSize - 1))) == 1
    }

    pub fn flip(&mut self, i: u32) {
        if i >= self.len {
            self.set(i);
        }
        self.sets[(i >> log2WordSize) as usize] ^= 1 << (i & (wordSize - 1))
    }
}
