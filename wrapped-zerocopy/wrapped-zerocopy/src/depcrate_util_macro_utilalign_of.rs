// Generated macro for align_of (macro)
macro_rules! Depcrate_util_macro_utilalign_of {
() => {
// Module: crate::util::macro_util
// Provides: {"align_of"}
// Dependencies: {}
# [doc = " Computes alignment of `$ty: ?Sized`."] # [doc = ""] # [doc = " `align_of!` produces code which is valid in a `const` context."] # [cfg (__ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS)] # [doc (hidden)] # [macro_export] macro_rules ! align_of { ($ ty : ty) => { { # [repr (C)] struct OffsetOfTrailingIsAlignment { _byte : u8 , _trailing : $ ty , } trailing_field_offset ! (OffsetOfTrailingIsAlignment , _trailing) } } ; }
};
}
