// Generated macro for impl_661 (impl)
macro_rules! Depcrate_ffi_os_strimpl_661 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_661"}
// Dependencies: {}
# [stable (feature = "cow_from_osstr" , since = "1.28.0")] impl < 'a > From < & 'a OsStr > for Cow < 'a , OsStr > { # [doc = " Converts the string reference into a [`Cow::Borrowed`]."] # [inline] fn from (s : & 'a OsStr) -> Cow < 'a , OsStr > { Cow :: Borrowed (s) } }
};
}
