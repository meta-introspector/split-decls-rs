// Generated macro for impl_627 (impl)
macro_rules! Depcrate_ffi_os_strimpl_627 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_627"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + AsRef < OsStr > > From < & T > for OsString { # [doc = " Copies any value implementing <code>[AsRef]&lt;[OsStr]&gt;</code>"] # [doc = " into a newly allocated [`OsString`]."] fn from (s : & T) -> OsString { trait SpecToOsString { fn spec_to_os_string (& self) -> OsString ; } impl < T : AsRef < OsStr > > SpecToOsString for T { # [inline] default fn spec_to_os_string (& self) -> OsString { self . as_ref () . to_os_string () } } macro spec_str ($ T : ty) { impl SpecToOsString for $ T { # [inline] fn spec_to_os_string (& self) -> OsString { OsString :: from (String :: from (self)) } } } spec_str ! (str) ; spec_str ! (String) ; s . spec_to_os_string () } }
};
}
