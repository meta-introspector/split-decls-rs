// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_hash_indexOccupiedEntry {
() => {
// Module: crate::hash_index
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " [`OccupiedEntry`] is a view into an occupied entry in a [`HashIndex`]."] pub struct OccupiedEntry < 'h , K , V , H = RandomState > where H : BuildHasher , { hashindex : & 'h HashIndex < K , V , H > , locked_bucket : LockedBucket < K , V , () , INDEX > , entry_ptr : EntryPtr < 'h , K , V , INDEX > , }
};
}
