// Generated macro for impl_91 (impl)
macro_rules! Depcrate_hash_indeximpl_91 {
() => {
// Module: crate::hash_index
// Provides: {"impl_91"}
// Dependencies: {}
impl < 'h , K , V , H > Entry < 'h , K , V , H > where K : Eq + Hash , V : Default , H : BuildHasher , { # [doc = " Ensures a value is in the entry by inserting the default value if empty."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashIndex;"] # [doc = ""] # [doc = " let hashindex: HashIndex<u64, u32> = HashIndex::default();"] # [doc = " hashindex.entry_sync(11).or_default();"] # [doc = " assert_eq!(hashindex.peek_with(&11, |_, v| *v), Some(0));"] # [doc = " ```"] # [inline] pub fn or_default (self) -> OccupiedEntry < 'h , K , V , H > { match self { Self :: Occupied (o) => o , Self :: Vacant (v) => v . insert_entry (Default :: default ()) , } } }
};
}
