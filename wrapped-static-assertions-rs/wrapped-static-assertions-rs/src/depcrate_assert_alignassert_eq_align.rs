// Generated macro for assert_eq_align (macro)
macro_rules! Depcrate_assert_alignassert_eq_align {
() => {
// Module: crate::assert_align
// Provides: {"assert_eq_align"}
// Dependencies: {}
# [doc = " Asserts that the types' alignments are equal."] # [doc = ""] # [doc = " This macro has been deprecated in favor of"] # [doc = " [`assert_align_eq!`](macro.assert_align_eq.html)."] # [deprecated (since = "1.2.0" , note = "Please use the 'assert_align_eq' macro instead")] # [macro_export (local_inner_macros)] macro_rules ! assert_eq_align { ($ ($ t : tt) *) => { assert_align_eq ! ($ ($ t) *) ; } ; }
};
}
