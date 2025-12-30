// Generated macro for impl_696 (impl)
macro_rules! Depcrate_ffi_os_strimpl_696 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_696"}
// Dependencies: {}
# [stable (feature = "osstring_extend" , since = "1.52.0")] impl Extend < OsString > for OsString { # [inline] fn extend < T : IntoIterator < Item = OsString > > (& mut self , iter : T) { for s in iter { self . push (& s) ; } } }
};
}
