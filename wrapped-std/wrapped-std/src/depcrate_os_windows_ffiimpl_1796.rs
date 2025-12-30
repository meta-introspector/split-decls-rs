// Generated macro for impl_1796 (impl)
macro_rules! Depcrate_os_windows_ffiimpl_1796 {
() => {
// Module: crate::os::windows::ffi
// Provides: {"impl_1796"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl OsStringExt for OsString { fn from_wide (wide : & [u16]) -> OsString { FromInner :: from_inner (Buf { inner : Wtf8Buf :: from_wide (wide) }) } }
};
}
