// Generated macro for impl_656 (impl)
macro_rules! Depcrate_ffi_os_strimpl_656 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_656"}
// Dependencies: {}
# [stable (feature = "shared_from_mut_slice" , since = "1.84.0")] impl From < & mut OsStr > for Arc < OsStr > { # [doc = " Copies the string into a newly allocated <code>[Arc]&lt;[OsStr]&gt;</code>."] # [inline] fn from (s : & mut OsStr) -> Arc < OsStr > { Arc :: from (& * s) } }
};
}
