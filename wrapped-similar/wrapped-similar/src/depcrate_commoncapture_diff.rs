// Generated macro for capture_diff (function)
macro_rules! Depcrate_commoncapture_diff {
() => {
// Module: crate::common
// Provides: {"capture_diff"}
// Dependencies: {}
# [doc = " Creates a diff between old and new with the given algorithm capturing the ops."] # [doc = ""] # [doc = " This is like [`diff`](crate::algorithms::diff) but instead of using an"] # [doc = " arbitrary hook this will always use [`Compact`] + [`Replace`] + [`Capture`]"] # [doc = " and return the captured [`DiffOp`]s."] pub fn capture_diff < Old , New > (alg : Algorithm , old : & Old , old_range : Range < usize > , new : & New , new_range : Range < usize > ,) -> Vec < DiffOp > where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , Old :: Output : Hash + Eq + Ord , New :: Output : PartialEq < Old :: Output > + Hash + Eq + Ord , { capture_diff_deadline (alg , old , old_range , new , new_range , None) }
};
}
