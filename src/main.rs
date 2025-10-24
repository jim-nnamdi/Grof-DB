use std::path::Path;

use SDB::lsm;

use crate::bloom::{bloom::BloomFilter, bloom2::Bloom2};

mod memtable;

pub mod sstable;
pub mod bloom;
pub mod network;

#[allow(dead_code)]
fn manual_benchmarking_wal(){
    let new_node = lsm::WNode::new("jim", "sam");
    let new_wal = lsm::WAL::open("./data").unwrap();
    new_wal.lock().unwrap().append(&new_node).unwrap();
    
    let dur = lsm::bench(|| {
        lsm::WAL::replay("./data/wal-000000.log").unwrap();
    });
    println!("{:?}", dur);

    let dur = lsm::bench(|| {
        lsm::WAL::replay_two("./data").unwrap();
    });
    println!("{:?}", dur);
}

fn bloom_benchmarks() {
    let mut bits_bloom = BloomFilter::new(10, 2);
    let dur = lsm::bench(|| {
        bits_bloom.insert(&3);
        bits_bloom.contains(&3);
    });
    println!("{:?}", dur);
    let mut bloom2 = Bloom2::new(4, 2);
    let dur = lsm::bench(|| {
        bloom2.append(&3);
        bloom2.contains(&3);
    });
    println!("{:?}", dur);
}

fn main() {
    bloom_benchmarks();
}