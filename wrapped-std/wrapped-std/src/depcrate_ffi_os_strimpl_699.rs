// Generated macro for impl_699 (impl)
macro_rules! Depcrate_ffi_os_strimpl_699 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_699"}
// Dependencies: {}
# [stable (feature = "osstring_extend" , since = "1.52.0")] impl FromIterator < OsString > for OsString { # [inline] fn from_iter < I : IntoIterator < Item = OsString > > (iter : I) -> Self { let mut iterator = iter . into_iter () ; match iterator . next () { None => OsString :: new () , Some (mut buf) => { buf . extend (iterator) ; buf } } } }
};
}
