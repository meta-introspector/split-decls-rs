// Generated macro for impl_628 (impl)
macro_rules! Depcrate_ffi_os_strimpl_628 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_628"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl ops :: Index < ops :: RangeFull > for OsString { type Output = OsStr ; # [inline] fn index (& self , _index : ops :: RangeFull) -> & OsStr { OsStr :: from_inner (self . inner . as_slice ()) } }
};
}
