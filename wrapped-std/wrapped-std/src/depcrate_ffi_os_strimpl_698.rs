// Generated macro for impl_698 (impl)
macro_rules! Depcrate_ffi_os_strimpl_698 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_698"}
// Dependencies: {}
# [stable (feature = "osstring_extend" , since = "1.52.0")] impl < 'a > Extend < Cow < 'a , OsStr > > for OsString { # [inline] fn extend < T : IntoIterator < Item = Cow < 'a , OsStr > > > (& mut self , iter : T) { for s in iter { self . push (& s) ; } } }
};
}
