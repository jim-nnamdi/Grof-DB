pub mod memtable {
    use std::io::Result;
    use std::sync::{Arc, RwLock};
    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};
    use SDB::lsm::{WAL,WNode};

    pub struct MTable {
        DBuf: Arc<RwLock<BTreeMap<String, Option<String>>>>,
        size: Arc<RwLock<usize>>,
    }

    impl MTable {
        pub fn new(dir: impl Into<PathBuf>) -> Result<Vec<WNode>> {
            let wal = WAL::replay_two(dir.into());
            wal
        }
        pub fn put(&self, key: &str, val: &str){
            let wnode = WNode::new(key, val);
            let mut dat = self.DBuf.write().unwrap();
            dat.insert(wnode.key.into(), wnode.val);
            let mut s = self.size.write().unwrap();
            *s += key.len() + val.len();
        }

        pub fn remove(&self, key: &str) {
            let mut buf = self.DBuf.write().unwrap();
            if let Some(val) = buf.remove(key.into()) {
                let mut sz = self.size.write().unwrap();
                *sz -= key.len();
                if let Some(v) = val{
                    *sz -= v.len()
                }
            }
        }
    }
}
