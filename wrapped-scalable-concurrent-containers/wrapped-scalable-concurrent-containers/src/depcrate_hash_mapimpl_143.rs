// Generated macro for impl_143 (impl)
macro_rules! Depcrate_hash_mapimpl_143 {
() => {
// Module: crate::hash_map
// Provides: {"impl_143"}
// Dependencies: {}
impl < 'h , K , V , H > Entry < 'h , K , V , H > where K : Eq + Hash , V : Default , H : BuildHasher , { # [doc = " Ensures a value is in the entry by inserting the default value if empty."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashMap;"] # [doc = ""] # [doc = " let hashmap: HashMap<u64, u32> = HashMap::default();"] # [doc = " hashmap.entry_sync(11).or_default();"] # [doc = " assert_eq!(hashmap.read_sync(&11, |_, v| *v), Some(0));"] # [doc = " ```"] # [inline] pub fn or_default (self) -> OccupiedEntry < 'h , K , V , H > { match self { Self :: Occupied (o) => o , Self :: Vacant (v) => v . insert_entry (Default :: default ()) , } } }
};
}
