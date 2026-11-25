use std::collections::BTreeMap ;



//creating the memtable and its threshold
pub struct MemTable {
    map: BTreeMap<Vec<u8>, Option<Vec<u8>>>,
    approximate_size: usize,
}

impl MemTable {
    pub fn new() -> Self{
        Self {
        map:BTreeMap::new(),
        approximate_size:0,
        }
    }

}
