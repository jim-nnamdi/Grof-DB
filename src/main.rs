use SDB::lsm;

pub mod memtable;
pub mod sstable;

fn main() {
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