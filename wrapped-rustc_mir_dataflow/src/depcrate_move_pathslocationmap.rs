// Generated macro for LocationMap (struct)
macro_rules! Depcrate_move_pathsLocationMap {
() => {
// Module: crate::move_paths
// Provides: {"LocationMap"}
// Dependencies: {}
# [derive (Debug)] pub struct LocationMap < T > { # [doc = " Location-indexed (BasicBlock for outer index, index within BB"] # [doc = " for inner index) map."] pub (crate) map : IndexVec < BasicBlock , Vec < T > > , }
};
}
