// Generated macro for impl_659 (impl)
macro_rules! Depcrate_ffi_os_strimpl_659 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_659"}
// Dependencies: {}
# [stable (feature = "shared_from_mut_slice" , since = "1.84.0")] impl From < & mut OsStr > for Rc < OsStr > { # [doc = " Copies the string into a newly allocated <code>[Rc]&lt;[OsStr]&gt;</code>."] # [inline] fn from (s : & mut OsStr) -> Rc < OsStr > { Rc :: from (& * s) } }
};
}
