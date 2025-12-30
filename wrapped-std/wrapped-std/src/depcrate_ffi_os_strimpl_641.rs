// Generated macro for impl_641 (impl)
macro_rules! Depcrate_ffi_os_strimpl_641 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_641"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl PartialOrd for OsString { # [inline] fn partial_cmp (& self , other : & OsString) -> Option < cmp :: Ordering > { (& * * self) . partial_cmp (& * * other) } # [inline] fn lt (& self , other : & OsString) -> bool { & * * self < & * * other } # [inline] fn le (& self , other : & OsString) -> bool { & * * self <= & * * other } # [inline] fn gt (& self , other : & OsString) -> bool { & * * self > & * * other } # [inline] fn ge (& self , other : & OsString) -> bool { & * * self >= & * * other } }
};
}
