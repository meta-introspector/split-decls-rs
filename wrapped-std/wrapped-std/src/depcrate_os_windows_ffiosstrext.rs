// Generated macro for OsStrExt (trait)
macro_rules! Depcrate_os_windows_ffiOsStrExt {
() => {
// Module: crate::os::windows::ffi
// Provides: {"OsStrExt"}
// Dependencies: {}
# [doc = " Windows-specific extensions to [`OsStr`]."] # [doc = ""] # [doc = " This trait is sealed: it cannot be implemented outside the standard library."] # [doc = " This is so that future additional methods are not breaking changes."] # [stable (feature = "rust1" , since = "1.0.0")] pub trait OsStrExt : Sealed { # [doc = " Re-encodes an `OsStr` as a wide character sequence, i.e., potentially"] # [doc = " ill-formed UTF-16."] # [doc = ""] # [doc = " This is lossless: calling [`OsStringExt::from_wide`] and then"] # [doc = " `encode_wide` on the result will yield the original code units."] # [doc = " Note that the encoding does not add a final null terminator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::ffi::OsString;"] # [doc = " use std::os::windows::prelude::*;"] # [doc = ""] # [doc = " // UTF-16 encoding for \"Unicode\"."] # [doc = " let source = [0x0055, 0x006E, 0x0069, 0x0063, 0x006F, 0x0064, 0x0065];"] # [doc = ""] # [doc = " let string = OsString::from_wide(&source[..]);"] # [doc = ""] # [doc = " let result: Vec<u16> = string.encode_wide().collect();"] # [doc = " assert_eq!(&source[..], &result[..]);"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] fn encode_wide (& self) -> EncodeWide < '_ > ; }
};
}
