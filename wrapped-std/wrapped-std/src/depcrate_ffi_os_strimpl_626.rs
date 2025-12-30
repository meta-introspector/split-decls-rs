// Generated macro for impl_626 (impl)
macro_rules! Depcrate_ffi_os_strimpl_626 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_626"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl From < String > for OsString { # [doc = " Converts a [`String`] into an [`OsString`]."] # [doc = ""] # [doc = " This conversion does not allocate or copy memory."] # [inline] fn from (s : String) -> OsString { OsString { inner : Buf :: from_string (s) } } }
};
}
