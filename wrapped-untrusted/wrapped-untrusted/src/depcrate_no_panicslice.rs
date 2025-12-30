// Generated macro for Slice (struct)
macro_rules! Depcrate_no_panicSlice {
() => {
// Module: crate::no_panic
// Provides: {"Slice"}
// Dependencies: {}
# [doc = " A wrapper around a slice that exposes no functions that can panic."] # [doc = ""] # [doc = " Intentionally avoids implementing `Debug`, `Eq`, and `PartialEq` to avoid"] # [doc = " creating a side channel that would leak information about the value."] # [derive (Clone , Copy)] pub struct Slice < 'a > { bytes : & 'a [u8] , }
};
}
