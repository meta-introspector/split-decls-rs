// Generated macro for impl_151 (impl)
macro_rules! Depcrate_hash_mapimpl_151 {
() => {
// Module: crate::hash_map
// Provides: {"impl_151"}
// Dependencies: {}
impl < K , V > ConsumableEntry < '_ , '_ , K , V > { # [doc = " Consumes the entry by moving out the key and value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashMap;"] # [doc = ""] # [doc = " let hashmap: HashMap<u64, u32> = HashMap::default();"] # [doc = ""] # [doc = " assert!(hashmap.insert_sync(1, 0).is_ok());"] # [doc = " assert!(hashmap.insert_sync(2, 1).is_ok());"] # [doc = " assert!(hashmap.insert_sync(3, 2).is_ok());"] # [doc = ""] # [doc = " let mut consumed = None;"] # [doc = ""] # [doc = " hashmap.iter_mut_sync(|entry| {"] # [doc = "     if entry.0 == 1 {"] # [doc = "         consumed.replace(entry.consume().1);"] # [doc = "     }"] # [doc = "     true"] # [doc = " });"] # [doc = ""] # [doc = " assert!(!hashmap.contains_sync(&1));"] # [doc = " assert_eq!(consumed, Some(0));"] # [doc = " ```"] # [inline] # [must_use] pub fn consume (self) -> (K , V) { * self . remove_probe |= true ; self . locked_bucket . writer . remove (self . locked_bucket . data_block , self . entry_ptr , self . guard) } }
};
}
