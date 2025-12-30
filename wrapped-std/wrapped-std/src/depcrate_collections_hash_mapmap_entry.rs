// Generated macro for map_entry (function)
macro_rules! Depcrate_collections_hash_mapmap_entry {
() => {
// Module: crate::collections::hash::map
// Provides: {"map_entry"}
// Dependencies: {}
# [inline] fn map_entry < 'a , K : 'a , V : 'a > (raw : base :: RustcEntry < 'a , K , V >) -> Entry < 'a , K , V > { match raw { base :: RustcEntry :: Occupied (base) => Entry :: Occupied (OccupiedEntry { base }) , base :: RustcEntry :: Vacant (base) => Entry :: Vacant (VacantEntry { base }) , } }
};
}
