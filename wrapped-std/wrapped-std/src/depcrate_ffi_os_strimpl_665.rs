// Generated macro for impl_665 (impl)
macro_rules! Depcrate_ffi_os_strimpl_665 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_665"}
// Dependencies: {}
# [stable (feature = "box_default_extra" , since = "1.17.0")] impl Default for Box < OsStr > { # [inline] fn default () -> Box < OsStr > { let rw = Box :: into_raw (Slice :: empty_box ()) as * mut OsStr ; unsafe { Box :: from_raw (rw) } } }
};
}
