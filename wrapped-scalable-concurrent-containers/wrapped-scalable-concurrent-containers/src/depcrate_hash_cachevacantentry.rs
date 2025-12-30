// Generated macro for VacantEntry (struct)
macro_rules! Depcrate_hash_cacheVacantEntry {
() => {
// Module: crate::hash_cache
// Provides: {"VacantEntry"}
// Dependencies: {}
# [doc = " [`VacantEntry`] is a view into a vacant cache entry in a [`HashCache`]."] pub struct VacantEntry < 'h , K , V , H = RandomState > where H : BuildHasher , { hashcache : & 'h HashCache < K , V , H > , key : K , hash : u64 , locked_bucket : LockedBucket < K , V , DoublyLinkedList , CACHE > , }
};
}
