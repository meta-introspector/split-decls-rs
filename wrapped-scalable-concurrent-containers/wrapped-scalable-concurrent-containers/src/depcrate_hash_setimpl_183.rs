// Generated macro for impl_183 (impl)
macro_rules! Depcrate_hash_setimpl_183 {
() => {
// Module: crate::hash_set
// Provides: {"impl_183"}
// Dependencies: {}
impl < K > ConsumableEntry < '_ , '_ , K > { # [doc = " Consumes the entry by moving out the key."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashSet;"] # [doc = ""] # [doc = " let hashset: HashSet<u64> = HashSet::default();"] # [doc = ""] # [doc = " assert!(hashset.insert_sync(1).is_ok());"] # [doc = " assert!(hashset.insert_sync(2).is_ok());"] # [doc = " assert!(hashset.insert_sync(3).is_ok());"] # [doc = ""] # [doc = " let mut consumed = None;"] # [doc = ""] # [doc = " hashset.iter_mut_sync(|entry| {"] # [doc = "     if *entry == 1 {"] # [doc = "         consumed.replace(entry.consume());"] # [doc = "     }"] # [doc = "     true"] # [doc = " });"] # [doc = ""] # [doc = " assert!(!hashset.contains_sync(&1));"] # [doc = " assert_eq!(consumed, Some(1));"] # [doc = " ```"] # [inline] # [must_use] pub fn consume (self) -> K { self . consumable . consume () . 0 } }
};
}
