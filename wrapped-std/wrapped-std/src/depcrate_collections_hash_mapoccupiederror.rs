// Generated macro for OccupiedError (struct)
macro_rules! Depcrate_collections_hash_mapOccupiedError {
() => {
// Module: crate::collections::hash::map
// Provides: {"OccupiedError"}
// Dependencies: {}
# [doc = " The error returned by [`try_insert`](HashMap::try_insert) when the key already exists."] # [doc = ""] # [doc = " Contains the occupied entry, and the value that was not inserted."] # [unstable (feature = "map_try_insert" , issue = "82766")] pub struct OccupiedError < 'a , K : 'a , V : 'a > { # [doc = " The entry in the map that was already occupied."] pub entry : OccupiedEntry < 'a , K , V > , # [doc = " The value which was not inserted, because the entry was already occupied."] pub value : V , }
};
}
