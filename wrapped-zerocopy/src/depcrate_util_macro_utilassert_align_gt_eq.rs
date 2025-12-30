// Generated macro for assert_align_gt_eq (macro)
macro_rules! Depcrate_util_macro_utilassert_align_gt_eq {
() => {
// Module: crate::util::macro_util
// Provides: {"assert_align_gt_eq"}
// Dependencies: {}
# [doc = " Does `t` have alignment greater than or equal to `u`?  If not, this macro"] # [doc = " produces a compile error. It must be invoked in a dead codepath. This is"] # [doc = " used in `transmute_ref!` and `transmute_mut!`."] # [doc (hidden)] # [macro_export] macro_rules ! assert_align_gt_eq { ($ t : ident , $ u : ident) => { { if false { let align_of : $ crate :: util :: macro_util :: AlignOf < _ > = unreachable ! () ; $ t = align_of . into_t () ; let mut max_aligns = $ crate :: util :: macro_util :: MaxAlignsOf :: new ($ t , $ u) ; max_aligns = unsafe { # [allow (clippy :: missing_transmute_annotations)] $ crate :: util :: macro_util :: core_reexport :: mem :: transmute (align_of) } ; } else { loop { } } } } ; }
};
}
