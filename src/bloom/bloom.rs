use std::{hash::{DefaultHasher, Hash, Hasher}};


pub struct BloomFilter {
    bits: Vec<u8>,
    size: u64,
    bfns: u64
}

impl BloomFilter {
    pub fn new(size: u64, k: u64) -> Self {
        let byt =  (size as usize  + 7) / 8;
        Self { bits: vec![0; byt], size, bfns: k }
    }

    pub fn hash<T:Hash>(&self, item: &T, seed: u64) -> u64 {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        item.hash(&mut hasher);
        hasher.finish() % self.size
    }

    pub fn setbit(&mut self, index: u64) {
        let byte_siz = (index / 8) as usize;
        let bit_siz = (index % 8) as u8;
        self.bits[byte_siz] |= 1 << bit_siz
    }

    pub fn getbit(&self, index:u64) -> bool {
        let byt_size = (index / 8) as usize;
        let bit_size = (index % 8) as u8;
        (self.bits[byt_size] & (1 << bit_size)) != 0
    }

    pub fn insert<T:Hash>(&mut self, item: &T) {
        for i in 0..self.bfns {
            let h = self.hash(item, i);
            self.setbit(h);
        }
    }

    pub fn contains<T:Hash>(&self, item:&T) -> bool {
        for i in 0..self.bfns {
            let h = self.hash(item, i);
            if !self.getbit(h) {
                return false;
            }
        }
        true
    }
}