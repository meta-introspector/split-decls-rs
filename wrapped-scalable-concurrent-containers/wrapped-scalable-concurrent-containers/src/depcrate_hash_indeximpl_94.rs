// Generated macro for impl_94 (impl)
macro_rules! Depcrate_hash_indeximpl_94 {
() => {
// Module: crate::hash_index
// Provides: {"impl_94"}
// Dependencies: {}
impl < K , V , H > OccupiedEntry < '_ , K , V , H > where K : Clone + Eq + Hash , H : BuildHasher , { # [doc = " Updates the entry by inserting a new entry and marking the existing entry removed."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::HashIndex;"] # [doc = " use scc::hash_index::Entry;"] # [doc = ""] # [doc = " let hashindex: HashIndex<u64, u32> = HashIndex::default();"] # [doc = ""] # [doc = " hashindex.entry_sync(37).or_insert(11);"] # [doc = ""] # [doc = " if let Entry::Occupied(mut o) = hashindex.entry_sync(37) {"] # [doc = "     o.update(29);"] # [doc = "     assert_eq!(o.get(), &29);"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(hashindex.peek_with(&37, |_, v| *v), Some(29));"] # [doc = " ```"] # [inline] pub fn update (& mut self , val : V) { let key = self . key () . clone () ; let partial_hash = self . entry_ptr . partial_hash (& self . locked_bucket . writer) ; let guard = Guard :: new () ; let prolonged_guard = self . hashindex . prolonged_guard_ref (& guard) ; let entry_ptr = self . locked_bucket . insert (u64 :: from (partial_hash) , (key , val) , prolonged_guard) ; self . locked_bucket . writer . mark_removed (& mut self . entry_ptr , prolonged_guard) ; self . entry_ptr = entry_ptr ; } }
};
}
