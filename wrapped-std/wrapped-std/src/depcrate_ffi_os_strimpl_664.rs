// Generated macro for impl_664 (impl)
macro_rules! Depcrate_ffi_os_strimpl_664 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_664"}
// Dependencies: {}
# [stable (feature = "str_tryfrom_osstr_impl" , since = "1.72.0")] impl < 'a > TryFrom < & 'a OsStr > for & 'a str { type Error = crate :: str :: Utf8Error ; # [doc = " Tries to convert an `&OsStr` to a `&str`."] # [doc = ""] # [doc = " ```"] # [doc = " use std::ffi::OsStr;"] # [doc = ""] # [doc = " let os_str = OsStr::new(\"foo\");"] # [doc = " let as_str = <&str>::try_from(os_str).unwrap();"] # [doc = " assert_eq!(as_str, \"foo\");"] # [doc = " ```"] fn try_from (value : & 'a OsStr) -> Result < Self , Self :: Error > { value . inner . to_str () } }
};
}
