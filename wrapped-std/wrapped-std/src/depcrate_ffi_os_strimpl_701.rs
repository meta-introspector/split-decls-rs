// Generated macro for impl_701 (impl)
macro_rules! Depcrate_ffi_os_strimpl_701 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_701"}
// Dependencies: {}
# [stable (feature = "osstring_extend" , since = "1.52.0")] impl < 'a > FromIterator < Cow < 'a , OsStr > > for OsString { # [inline] fn from_iter < I : IntoIterator < Item = Cow < 'a , OsStr > > > (iter : I) -> Self { let mut iterator = iter . into_iter () ; match iterator . next () { None => OsString :: new () , Some (Cow :: Owned (mut buf)) => { buf . extend (iterator) ; buf } Some (Cow :: Borrowed (buf)) => { let mut buf = OsString :: from (buf) ; buf . extend (iterator) ; buf } } } }
};
}
