// Generated macro for impl_642 (impl)
macro_rules! Depcrate_ffi_os_strimpl_642 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_642"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl PartialOrd < str > for OsString { # [inline] fn partial_cmp (& self , other : & str) -> Option < cmp :: Ordering > { (& * * self) . partial_cmp (other) } }
};
}
