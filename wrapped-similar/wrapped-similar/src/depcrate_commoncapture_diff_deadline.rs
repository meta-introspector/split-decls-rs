// Generated macro for capture_diff_deadline (function)
macro_rules! Depcrate_commoncapture_diff_deadline {
() => {
// Module: crate::common
// Provides: {"capture_diff_deadline"}
// Dependencies: {}
# [doc = " Creates a diff between old and new with the given algorithm capturing the ops."] # [doc = ""] # [doc = " Works like [`capture_diff`] but with an optional deadline."] pub fn capture_diff_deadline < Old , New > (alg : Algorithm , old : & Old , old_range : Range < usize > , new : & New , new_range : Range < usize > , deadline : Option < Instant > ,) -> Vec < DiffOp > where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , Old :: Output : Hash + Eq + Ord , New :: Output : PartialEq < Old :: Output > + Hash + Eq + Ord , { let mut d = Compact :: new (Replace :: new (Capture :: new ()) , old , new) ; diff_deadline (alg , & mut d , old , old_range , new , new_range , deadline) . unwrap () ; d . into_inner () . into_inner () . into_ops () }
};
}
