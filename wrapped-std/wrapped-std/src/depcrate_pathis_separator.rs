// Generated macro for is_separator (function)
macro_rules! Depcrate_pathis_separator {
() => {
// Module: crate::path
// Provides: {"is_separator"}
// Dependencies: {}
# [doc = " Determines whether the character is one of the permitted path"] # [doc = " separators for the current platform."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::path;"] # [doc = ""] # [doc = " assert!(path::is_separator('/')); // '/' works for both Unix and Windows"] # [doc = " assert!(!path::is_separator('❤'));"] # [doc = " ```"] # [must_use] # [stable (feature = "rust1" , since = "1.0.0")] pub fn is_separator (c : char) -> bool { c . is_ascii () && is_sep_byte (c as u8) }
};
}
