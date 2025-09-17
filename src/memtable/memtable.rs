pub mod memtable {
    use std::io::Result;
    use std::sync::{Arc, RwLock};
    use std::collections::BTreeMap;
    use std::path::PathBuf;
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
        pub fn add(&self, key: &str, val: &str) {
            let wnode = WNode::new(key, val);
            let mut dat = self.DBuf.write().unwrap();
            dat.insert(wnode.key.into(), wnode.val);
            let mut s = self.size.write().unwrap();
            *s += key.len() + val.len();
        }
    }
}
