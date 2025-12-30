// Generated macro for impl_1420 (impl)
macro_rules! Depcrate_os_unix_ffi_os_strimpl_1420 {
() => {
// Module: crate::os::unix::ffi::os_str
// Provides: {"impl_1420"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl OsStringExt for OsString { # [inline] fn from_vec (vec : Vec < u8 >) -> OsString { FromInner :: from_inner (Buf { inner : vec }) } # [inline] fn into_vec (self) -> Vec < u8 > { self . into_inner () . inner } }
};
}
