// Generated macro for impl_667 (impl)
macro_rules! Depcrate_ffi_os_strimpl_667 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_667"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl PartialEq for OsStr { # [inline] fn eq (& self , other : & OsStr) -> bool { self . as_encoded_bytes () . eq (other . as_encoded_bytes ()) } }
};
}
