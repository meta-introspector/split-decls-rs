// Generated macro for HeaderSliceWithLengthProtected (struct)
macro_rules! Depcrate_headerHeaderSliceWithLengthProtected {
() => {
// Module: crate::header
// Provides: {"HeaderSliceWithLengthProtected"}
// Dependencies: {}
# [doc = " A type wrapping `HeaderSlice<HeaderWithLength<H>, T>` that is used internally in `ThinArc`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Safety-usable invariants:"] # [doc = ""] # [doc = " - This is guaranteed to have the same representation as `HeaderSlice<HeaderWithLength<H>, [T]>`"] # [doc = " - The header length (`.length()`) is checked to be the slice length"] # [derive (Debug , Hash , Eq , PartialEq , Ord , PartialOrd)] # [repr (transparent)] pub struct HeaderSliceWithLengthProtected < H , T > { inner : HeaderSliceWithLengthUnchecked < H , T > , }
};
}
