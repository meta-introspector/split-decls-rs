// Generated macro for OsStringExt (trait)
macro_rules! Depcrate_os_windows_ffiOsStringExt {
() => {
// Module: crate::os::windows::ffi
// Provides: {"OsStringExt"}
// Dependencies: {}
# [doc = " Windows-specific extensions to [`OsString`]."] # [doc = ""] # [doc = " This trait is sealed: it cannot be implemented outside the standard library."] # [doc = " This is so that future additional methods are not breaking changes."] # [stable (feature = "rust1" , since = "1.0.0")] pub trait OsStringExt : Sealed { # [doc = " Creates an `OsString` from a potentially ill-formed UTF-16 slice of"] # [doc = " 16-bit code units."] # [doc = ""] # [doc = " This is lossless: calling [`OsStrExt::encode_wide`] on the resulting string"] # [doc = " will always return the original code units."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::ffi::OsString;"] # [doc = " use std::os::windows::prelude::*;"] # [doc = ""] # [doc = " // UTF-16 encoding for \"Unicode\"."] # [doc = " let source = [0x0055, 0x006E, 0x0069, 0x0063, 0x006F, 0x0064, 0x0065];"] # [doc = ""] # [doc = " let string = OsString::from_wide(&source[..]);"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] fn from_wide (wide : & [u16]) -> Self ; }
};
}
