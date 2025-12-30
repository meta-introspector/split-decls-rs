// Generated macro for impl_660 (impl)
macro_rules! Depcrate_ffi_os_strimpl_660 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_660"}
// Dependencies: {}
# [stable (feature = "cow_from_osstr" , since = "1.28.0")] impl < 'a > From < OsString > for Cow < 'a , OsStr > { # [doc = " Moves the string into a [`Cow::Owned`]."] # [inline] fn from (s : OsString) -> Cow < 'a , OsStr > { Cow :: Owned (s) } }
};
}
