use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<K: PartialEq + Hash + Eq, V: Cacheable> Index<V> for IndexMap<K, V> {
    type Output = K;
    fn index(&self, index: V) -> &Self::Output {
        let (k, v) = self.index_map.get_index(index.to_index()).unwrap();
        assert_eq!(* v, index, "Provided value doesn't match with indexed value");
        k
    }
}
