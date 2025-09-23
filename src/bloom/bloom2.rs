use std::hash::{DefaultHasher, Hash, Hasher};


pub struct Bloom2 {
    bits: Vec<u8>,
    size: u64,
    hfns: u64
}

impl Bloom2 {
    pub fn new(size: u64, hfns: u64) -> Bloom2 {
        Bloom2 { bits: vec![0; size as usize], size, hfns }
    }

    pub fn hash<T:Hash>(&self, item: &T, seed: u64) -> u64 {
        let mut hashf = DefaultHasher::new();
        seed.hash(&mut hashf);
        item.hash(&mut hashf);
        hashf.finish() % self.size
    }

    pub fn append<T:Hash>(&mut self, item: &T) {
        for i in 0..self.hfns {
            let hsh = self.hash(item, i);
            self.bits[hsh as usize] = 1;
        }
    }

    pub fn contains<T:Hash>(&self, item: &T) -> bool {
        for i in 0..self.hfns {
            let hsh = self.hash(item, i);
            if self.bits[hsh as usize] != 0 {
                return true;
            }
        }
        false
    }
}