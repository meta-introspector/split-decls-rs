// Generated macro for impl_658 (impl)
macro_rules! Depcrate_ffi_os_strimpl_658 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_658"}
// Dependencies: {}
# [stable (feature = "shared_from_slice2" , since = "1.24.0")] impl From < & OsStr > for Rc < OsStr > { # [doc = " Copies the string into a newly allocated <code>[Rc]&lt;[OsStr]&gt;</code>."] # [inline] fn from (s : & OsStr) -> Rc < OsStr > { let rc = s . inner . into_rc () ; unsafe { Rc :: from_raw (Rc :: into_raw (rc) as * const OsStr) } } }
};
}
