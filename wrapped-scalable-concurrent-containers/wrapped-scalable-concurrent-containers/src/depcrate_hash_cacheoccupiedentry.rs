// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_hash_cacheOccupiedEntry {
() => {
// Module: crate::hash_cache
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " [`OccupiedEntry`] is a view into an occupied cache entry in a [`HashCache`]."] pub struct OccupiedEntry < 'h , K , V , H = RandomState > where H : BuildHasher , { hashcache : & 'h HashCache < K , V , H > , locked_bucket : LockedBucket < K , V , DoublyLinkedList , CACHE > , entry_ptr : EntryPtr < 'h , K , V , CACHE > , }
};
}
