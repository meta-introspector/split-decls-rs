// Generated macro for Entry (enum)
macro_rules! Depcrate_hash_cacheEntry {
() => {
// Module: crate::hash_cache
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " [`Entry`] represents a single cache entry in a [`HashCache`]."] pub enum Entry < 'h , K , V , H = RandomState > where H : BuildHasher , { # [doc = " An occupied entry."] Occupied (OccupiedEntry < 'h , K , V , H >) , # [doc = " A vacant entry."] Vacant (VacantEntry < 'h , K , V , H >) , }
};
}
