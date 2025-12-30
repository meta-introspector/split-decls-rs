// Generated macro for impl_1422 (impl)
macro_rules! Depcrate_os_unix_ffi_os_strimpl_1422 {
() => {
// Module: crate::os::unix::ffi::os_str
// Provides: {"impl_1422"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl OsStrExt for OsStr { # [inline] fn from_bytes (slice : & [u8]) -> & OsStr { unsafe { mem :: transmute (slice) } } # [inline] fn as_bytes (& self) -> & [u8] { & self . as_inner () . inner } }
};
}
