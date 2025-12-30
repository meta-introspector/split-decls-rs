// Generated macro for impl_143 (impl)
macro_rules! Depcrate_udiffimpl_143 {
() => {
// Module: crate::udiff
// Provides: {"impl_143"}
// Dependencies: {}
impl UnifiedHunkHeader { # [doc = " Creates a hunk header from a (non empty) slice of diff ops."] pub fn new (ops : & [DiffOp]) -> UnifiedHunkHeader { let first = ops [0] ; let last = ops [ops . len () - 1] ; let old_start = first . old_range () . start ; let new_start = first . new_range () . start ; let old_end = last . old_range () . end ; let new_end = last . new_range () . end ; UnifiedHunkHeader { old_range : UnifiedDiffHunkRange (old_start , old_end) , new_range : UnifiedDiffHunkRange (new_start , new_end) , } } }
};
}
