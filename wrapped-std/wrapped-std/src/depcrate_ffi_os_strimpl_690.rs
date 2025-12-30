// Generated macro for impl_690 (impl)
macro_rules! Depcrate_ffi_os_strimpl_690 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_690"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl AsRef < OsStr > for str { # [inline] fn as_ref (& self) -> & OsStr { OsStr :: from_inner (Slice :: from_str (self)) } }
};
}
