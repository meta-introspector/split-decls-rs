// Generated macro for Entry (enum)
macro_rules! Depcrate_collections_hash_mapEntry {
() => {
// Module: crate::collections::hash::map
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A view into a single entry in a map, which may either be vacant or occupied."] # [doc = ""] # [doc = " This `enum` is constructed from the [`entry`] method on [`HashMap`]."] # [doc = ""] # [doc = " [`entry`]: HashMap::entry"] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "HashMapEntry")] pub enum Entry < 'a , K : 'a , V : 'a > { # [doc = " An occupied entry."] # [stable (feature = "rust1" , since = "1.0.0")] Occupied (# [stable (feature = "rust1" , since = "1.0.0")] OccupiedEntry < 'a , K , V >) , # [doc = " A vacant entry."] # [stable (feature = "rust1" , since = "1.0.0")] Vacant (# [stable (feature = "rust1" , since = "1.0.0")] VacantEntry < 'a , K , V >) , }
};
}
