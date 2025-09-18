pub mod memtable {
    use std::io::Result;
    use std::sync::{Arc, RwLock};
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use SDB::lsm::{WAL,WNode};

    use crate::sstable::sstable;

    const MT_SIZ: u64 = 64 * 1024 * 1024;
    const MT_DIR: &str = "./data/memtable";

    pub struct MTable {
        dbuf: Arc<RwLock<BTreeMap<String, Option<String>>>>,
        size: Arc<RwLock<usize>>,
    }

    impl MTable {
        pub fn new(dir: impl Into<PathBuf>) -> Result<Vec<WNode>> {
            let wal = WAL::replay_two(dir.into());
            wal
        }
        pub fn put(&self, key: &str, val: &str){
            let sz =self.size.write().unwrap();
            if *sz as u64 >= MT_SIZ { 
                self.flush(MT_DIR).unwrap();
            }
            let wnode = WNode::new(key, val);
            let mut dat = self.dbuf.write().unwrap();
            dat.insert(wnode.key.into(), wnode.val);
            let mut s = self.size.write().unwrap();
            *s += key.len() + val.len();
        }

        pub fn remove(&self, key: &str) {
            let mut buf = self.dbuf.write().unwrap();
            if let Some(val) = buf.remove(key.into()) {
                let mut sz = self.size.write().unwrap();
                *sz -= key.len();
                if let Some(v) = val{
                    *sz -= v.len()
                }
            }
        }

        pub fn flush(&self, dir: impl Into<PathBuf>) -> Result<()>{
            let sz =  self.size.read().unwrap();
            if *sz as u64 >= MT_SIZ {
                let dir =  dir.into();
                let stable  = sstable::sstable::SSTable::new(&dir)?;
                stable.append_to_sstable(&dir);
            }
            Ok(())
        }
    }
}
