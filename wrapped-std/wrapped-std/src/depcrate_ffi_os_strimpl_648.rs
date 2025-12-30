// Generated macro for impl_648 (impl)
macro_rules! Depcrate_ffi_os_strimpl_648 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_648"}
// Dependencies: {}
# [stable (feature = "box_from_mut_slice" , since = "1.84.0")] impl From < & mut OsStr > for Box < OsStr > { # [doc = " Copies the string into a newly allocated <code>[Box]&lt;[OsStr]&gt;</code>."] # [inline] fn from (s : & mut OsStr) -> Box < OsStr > { Self :: from (& * s) } }
};
}
