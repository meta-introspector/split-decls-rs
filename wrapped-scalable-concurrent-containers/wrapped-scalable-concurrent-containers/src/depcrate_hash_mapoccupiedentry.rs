// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_hash_mapOccupiedEntry {
() => {
// Module: crate::hash_map
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " [`OccupiedEntry`] is a view into an occupied entry in a [`HashMap`]."] pub struct OccupiedEntry < 'h , K , V , H = RandomState > where H : BuildHasher , { hashmap : & 'h HashMap < K , V , H > , locked_bucket : LockedBucket < K , V , () , MAP > , entry_ptr : EntryPtr < 'h , K , V , MAP > , }
};
}
