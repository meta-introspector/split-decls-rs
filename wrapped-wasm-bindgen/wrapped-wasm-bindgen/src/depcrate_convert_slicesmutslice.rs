// Generated macro for MutSlice (struct)
macro_rules! Depcrate_convert_slicesMutSlice {
() => {
// Module: crate::convert::slices
// Provides: {"MutSlice"}
// Dependencies: {}
# [doc = " The representation of a mutable slice passed from JS to Rust."] pub struct MutSlice < T > { # [doc = " A copy of the data in the JS typed array."] contents : Box < [T] > , # [doc = " A reference to the original JS typed array."] js : JsValue , }
};
}
