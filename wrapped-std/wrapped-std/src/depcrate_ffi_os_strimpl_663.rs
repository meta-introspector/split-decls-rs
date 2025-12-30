// Generated macro for impl_663 (impl)
macro_rules! Depcrate_ffi_os_strimpl_663 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_663"}
// Dependencies: {}
# [stable (feature = "osstring_from_cow_osstr" , since = "1.28.0")] impl < 'a > From < Cow < 'a , OsStr > > for OsString { # [doc = " Converts a `Cow<'a, OsStr>` into an [`OsString`],"] # [doc = " by copying the contents if they are borrowed."] # [inline] fn from (s : Cow < 'a , OsStr >) -> Self { s . into_owned () } }
};
}
