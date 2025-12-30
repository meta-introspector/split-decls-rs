// Generated macro for capture_diff_slices_deadline (function)
macro_rules! Depcrate_commoncapture_diff_slices_deadline {
() => {
// Module: crate::common
// Provides: {"capture_diff_slices_deadline"}
// Dependencies: {}
# [doc = " Creates a diff between old and new with the given algorithm capturing the ops."] # [doc = ""] # [doc = " Works like [`capture_diff_slices`] but with an optional deadline."] pub fn capture_diff_slices_deadline < T > (alg : Algorithm , old : & [T] , new : & [T] , deadline : Option < Instant > ,) -> Vec < DiffOp > where T : Eq + Hash + Ord , { capture_diff_deadline (alg , old , 0 .. old . len () , new , 0 .. new . len () , deadline) }
};
}
