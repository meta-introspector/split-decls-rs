// Generated macro for RustStringInner (struct)
macro_rules! DepcrateRustStringInner {
() => {
// Module: crate
// Provides: {"RustStringInner"}
// Dependencies: {}
# [doc = " Underlying implementation of [`RustString`]."] # [doc = ""] # [doc = " Having two separate types makes it possible to use the opaque [`RustString`]"] # [doc = " in FFI signatures without `improper_ctypes` warnings. This is a workaround"] # [doc = " for the fact that there is no way to opt out of `improper_ctypes` when"] # [doc = " _declaring_ a type (as opposed to using that type)."] # [derive (Default)] struct RustStringInner { bytes : RefCell < Vec < u8 > > , }
};
}
