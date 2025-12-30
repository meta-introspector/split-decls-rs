// Generated macro for impl_673 (impl)
macro_rules! Depcrate_ffi_os_strimpl_673 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_673"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Ord for OsStr { # [inline] fn cmp (& self , other : & OsStr) -> cmp :: Ordering { self . as_encoded_bytes () . cmp (other . as_encoded_bytes ()) } }
};
}
