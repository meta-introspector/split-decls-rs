// Generated macro for OsStr (struct)
macro_rules! Depcrate_ffi_os_strOsStr {
() => {
// Module: crate::ffi::os_str
// Provides: {"OsStr"}
// Dependencies: {}
# [doc = " Borrowed reference to an OS string (see [`OsString`])."] # [doc = ""] # [doc = " This type represents a borrowed reference to a string in the operating system's preferred"] # [doc = " representation."] # [doc = ""] # [doc = " `&OsStr` is to [`OsString`] as <code>&[str]</code> is to [`String`]: the"] # [doc = " former in each pair are borrowed references; the latter are owned strings."] # [doc = ""] # [doc = " See the [module's toplevel documentation about conversions][conversions] for a discussion on"] # [doc = " the traits which `OsStr` implements for [conversions] from/to native representations."] # [doc = ""] # [doc = " [conversions]: super#conversions"] # [cfg_attr (not (test) , rustc_diagnostic_item = "OsStr")] # [stable (feature = "rust1" , since = "1.0.0")] # [repr (transparent)] pub struct OsStr { inner : Slice , }
};
}
