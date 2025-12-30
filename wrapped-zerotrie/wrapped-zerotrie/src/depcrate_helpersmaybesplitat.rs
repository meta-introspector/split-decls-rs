// Generated macro for MaybeSplitAt (trait)
macro_rules! Depcrate_helpersMaybeSplitAt {
() => {
// Module: crate::helpers
// Provides: {"MaybeSplitAt"}
// Dependencies: {}
pub (crate) trait MaybeSplitAt < T > { # [doc = " Like slice::split_at but debug-panics and returns an empty second slice"] # [doc = " if the index is out of range."] fn debug_split_at (& self , mid : usize) -> (& Self , & Self) ; }
};
}
