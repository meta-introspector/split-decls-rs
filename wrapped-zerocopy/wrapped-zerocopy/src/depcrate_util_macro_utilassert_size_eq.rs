// Generated macro for assert_size_eq (macro)
macro_rules! Depcrate_util_macro_utilassert_size_eq {
() => {
// Module: crate::util::macro_util
// Provides: {"assert_size_eq"}
// Dependencies: {}
# [doc = " Do `t` and `u` have the same size?  If not, this macro produces a compile"] # [doc = " error. It must be invoked in a dead codepath. This is used in"] # [doc = " `transmute_ref!` and `transmute_mut!`."] # [doc (hidden)] # [macro_export] macro_rules ! assert_size_eq { ($ t : ident , $ u : ident) => { { if false { $ u = unsafe { # [allow (clippy :: useless_transmute , clippy :: missing_transmute_annotations)] $ crate :: util :: macro_util :: core_reexport :: mem :: transmute ($ t) } ; } else { loop { } } } } ; }
};
}
