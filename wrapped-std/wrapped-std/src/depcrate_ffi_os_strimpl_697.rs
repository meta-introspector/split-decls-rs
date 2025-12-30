// Generated macro for impl_697 (impl)
macro_rules! Depcrate_ffi_os_strimpl_697 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_697"}
// Dependencies: {}
# [stable (feature = "osstring_extend" , since = "1.52.0")] impl < 'a > Extend < & 'a OsStr > for OsString { # [inline] fn extend < T : IntoIterator < Item = & 'a OsStr > > (& mut self , iter : T) { for s in iter { self . push (s) ; } } }
};
}
