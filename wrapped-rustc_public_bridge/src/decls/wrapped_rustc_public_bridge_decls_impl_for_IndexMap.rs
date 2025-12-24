use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<K: PartialEq + Hash + Eq, V: Cacheable> IndexMap<K, V> {
    pub fn create_or_fetch(&mut self, key: K) -> V {
        let len = self.index_map.len();
        let v = self.index_map.entry(key).or_insert(V::to_val(len));
        *v
    }
}
