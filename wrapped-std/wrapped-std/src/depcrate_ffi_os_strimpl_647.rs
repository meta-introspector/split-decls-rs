// Generated macro for impl_647 (impl)
macro_rules! Depcrate_ffi_os_strimpl_647 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_647"}
// Dependencies: {}
# [stable (feature = "box_from_os_str" , since = "1.17.0")] impl From < & OsStr > for Box < OsStr > { # [doc = " Copies the string into a newly allocated <code>[Box]&lt;[OsStr]&gt;</code>."] # [inline] fn from (s : & OsStr) -> Box < OsStr > { let rw = Box :: into_raw (s . inner . into_box ()) as * mut OsStr ; unsafe { Box :: from_raw (rw) } } }
};
}
