// Generated macro for impl_671 (impl)
macro_rules! Depcrate_ffi_os_strimpl_671 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_671"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl PartialOrd for OsStr { # [inline] fn partial_cmp (& self , other : & OsStr) -> Option < cmp :: Ordering > { self . as_encoded_bytes () . partial_cmp (other . as_encoded_bytes ()) } # [inline] fn lt (& self , other : & OsStr) -> bool { self . as_encoded_bytes () . lt (other . as_encoded_bytes ()) } # [inline] fn le (& self , other : & OsStr) -> bool { self . as_encoded_bytes () . le (other . as_encoded_bytes ()) } # [inline] fn gt (& self , other : & OsStr) -> bool { self . as_encoded_bytes () . gt (other . as_encoded_bytes ()) } # [inline] fn ge (& self , other : & OsStr) -> bool { self . as_encoded_bytes () . ge (other . as_encoded_bytes ()) } }
};
}
