// Generated macro for AsChar (trait)
macro_rules! Depcrate_streamAsChar {
() => {
// Module: crate::stream
// Provides: {"AsChar"}
// Dependencies: {}
# [doc = " Transforms a token into a char for basic string parsing"] # [allow (clippy :: len_without_is_empty)] # [allow (clippy :: wrong_self_convention)] pub trait AsChar { # [doc = " Makes a char from self"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use winnow::prelude::*;"] # [doc = ""] # [doc = " assert_eq!('a'.as_char(), 'a');"] # [doc = " assert_eq!(u8::MAX.as_char(), std::char::from_u32(u8::MAX as u32).unwrap());"] # [doc = " ```"] fn as_char (self) -> char ; # [doc = " Tests that self is an alphabetic character"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **Warning:** for `&str` it matches alphabetic"] # [doc = " characters outside of the 52 ASCII letters"] # [doc = ""] # [doc = " </div>"] fn is_alpha (self) -> bool ; # [doc = " Tests that self is an alphabetic character"] # [doc = " or a decimal digit"] fn is_alphanum (self) -> bool ; # [doc = " Tests that self is a decimal digit"] fn is_dec_digit (self) -> bool ; # [doc = " Tests that self is an hex digit"] fn is_hex_digit (self) -> bool ; # [doc = " Tests that self is an octal digit"] fn is_oct_digit (self) -> bool ; # [doc = " Gets the len in bytes for self"] fn len (self) -> usize ; # [doc = " Tests that self is ASCII space or tab"] fn is_space (self) -> bool ; # [doc = " Tests if byte is ASCII newline: \\n"] fn is_newline (self) -> bool ; }
};
}
