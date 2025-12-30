// Generated macro for Entry (enum)
macro_rules! Depcrate_hash_mapEntry {
() => {
// Module: crate::hash_map
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " [`Entry`] represents a single entry in a [`HashMap`]."] pub enum Entry < 'h , K , V , H = RandomState > where H : BuildHasher , { # [doc = " An occupied entry."] Occupied (OccupiedEntry < 'h , K , V , H >) , # [doc = " A vacant entry."] Vacant (VacantEntry < 'h , K , V , H >) , }
};
}
