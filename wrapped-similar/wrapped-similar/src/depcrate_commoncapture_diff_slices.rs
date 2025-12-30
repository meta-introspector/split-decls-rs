// Generated macro for capture_diff_slices (function)
macro_rules! Depcrate_commoncapture_diff_slices {
() => {
// Module: crate::common
// Provides: {"capture_diff_slices"}
// Dependencies: {}
# [doc = " Creates a diff between old and new with the given algorithm capturing the ops."] pub fn capture_diff_slices < T > (alg : Algorithm , old : & [T] , new : & [T]) -> Vec < DiffOp > where T : Eq + Hash + Ord , { capture_diff_slices_deadline (alg , old , new , None) }
};
}
