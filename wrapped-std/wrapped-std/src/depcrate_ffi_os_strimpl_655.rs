// Generated macro for impl_655 (impl)
macro_rules! Depcrate_ffi_os_strimpl_655 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_655"}
// Dependencies: {}
# [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < & OsStr > for Arc < OsStr > { # [doc = " Copies the string into a newly allocated <code>[Arc]&lt;[OsStr]&gt;</code>."] # [inline] fn from (s : & OsStr) -> Arc < OsStr > { let arc = s . inner . into_arc () ; unsafe { Arc :: from_raw (Arc :: into_raw (arc) as * const OsStr) } } }
};
}
