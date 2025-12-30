// Generated macro for Entry (enum)
macro_rules! Depcrate_anymapEntry {
() => {
// Module: crate::anymap
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A view into a single location in an `Map`, which may be vacant or occupied."] pub enum Entry < 'map , A : ? Sized + Downcast , V > { # [doc = " An occupied Entry"] Occupied (OccupiedEntry < 'map , A , V >) , # [doc = " A vacant Entry"] Vacant (VacantEntry < 'map , A , V >) , }
};
}
