// Generated macro for impl_695 (impl)
macro_rules! Depcrate_ffi_os_strimpl_695 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_695"}
// Dependencies: {}
# [stable (feature = "osstring_from_str" , since = "1.45.0")] impl FromStr for OsString { type Err = core :: convert :: Infallible ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (OsString :: from (s)) } }
};
}
