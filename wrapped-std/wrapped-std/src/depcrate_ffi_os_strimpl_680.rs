// Generated macro for impl_680 (impl)
macro_rules! Depcrate_ffi_os_strimpl_680 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_680"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl Hash for OsStr { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . as_encoded_bytes () . hash (state) } }
};
}
