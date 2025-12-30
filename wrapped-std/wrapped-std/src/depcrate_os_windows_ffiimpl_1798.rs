// Generated macro for impl_1798 (impl)
macro_rules! Depcrate_os_windows_ffiimpl_1798 {
() => {
// Module: crate::os::windows::ffi
// Provides: {"impl_1798"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl OsStrExt for OsStr { # [inline] fn encode_wide (& self) -> EncodeWide < '_ > { EncodeWide { inner : self . as_inner () . inner . encode_wide () } } }
};
}
