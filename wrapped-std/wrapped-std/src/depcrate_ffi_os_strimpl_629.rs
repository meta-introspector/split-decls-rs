// Generated macro for impl_629 (impl)
macro_rules! Depcrate_ffi_os_strimpl_629 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_629"}
// Dependencies: {}
# [stable (feature = "mut_osstr" , since = "1.44.0")] impl ops :: IndexMut < ops :: RangeFull > for OsString { # [inline] fn index_mut (& mut self , _index : ops :: RangeFull) -> & mut OsStr { OsStr :: from_inner_mut (self . inner . as_mut_slice ()) } }
};
}
