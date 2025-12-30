// Generated macro for SliceFormat (trait)
macro_rules! Depcrate_fmtSliceFormat {
() => {
// Module: crate::fmt
// Provides: {"SliceFormat"}
// Dependencies: {}
# [doc = " Indicates a format which corresponds to a Rust slice type,"] # [doc = " representing exactly the same invariants."] pub unsafe trait SliceFormat : Format + Sized { type Slice : ? Sized + Slice ; }
};
}
