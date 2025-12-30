// Generated macro for stable (function)
macro_rules! Depcrate_rustc_internalstable {
() => {
// Module: crate::rustc_internal
// Provides: {"stable"}
// Dependencies: {}
# [doc = " Convert an internal Rust compiler item into its stable counterpart, if one exists."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This function is unstable, and its behavior may change at any point."] # [doc = " E.g.: Items that were previously supported, may no longer be supported, or its translation may"] # [doc = " change."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic if rustc_public has not been properly initialized."] pub fn stable < 'tcx , S : Stable < 'tcx > > (item : S) -> S :: T { with_container (| tables , cx | item . stable (tables , cx)) }
};
}
