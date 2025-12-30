// Generated macro for OccupiedEntry (struct)
macro_rules! Depcrate_anymapOccupiedEntry {
() => {
// Module: crate::anymap
// Provides: {"OccupiedEntry"}
// Dependencies: {}
# [doc = " A view into a single occupied location in an `Map`."] pub struct OccupiedEntry < 'map , A : ? Sized + Downcast , V : 'map > { inner : hash_map :: OccupiedEntry < 'map , TypeId , Box < A > > , type_ : PhantomData < V > , }
};
}
