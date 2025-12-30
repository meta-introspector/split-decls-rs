// Generated macro for impl_700 (impl)
macro_rules! Depcrate_ffi_os_strimpl_700 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_700"}
// Dependencies: {}
# [stable (feature = "osstring_extend" , since = "1.52.0")] impl < 'a > FromIterator < & 'a OsStr > for OsString { # [inline] fn from_iter < I : IntoIterator < Item = & 'a OsStr > > (iter : I) -> Self { let mut buf = Self :: new () ; for s in iter { buf . push (s) ; } buf } }
};
}
