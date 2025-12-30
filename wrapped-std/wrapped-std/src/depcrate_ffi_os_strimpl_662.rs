// Generated macro for impl_662 (impl)
macro_rules! Depcrate_ffi_os_strimpl_662 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_662"}
// Dependencies: {}
# [stable (feature = "cow_from_osstr" , since = "1.28.0")] impl < 'a > From < & 'a OsString > for Cow < 'a , OsStr > { # [doc = " Converts the string reference into a [`Cow::Borrowed`]."] # [inline] fn from (s : & 'a OsString) -> Cow < 'a , OsStr > { Cow :: Borrowed (s . as_os_str ()) } }
};
}
