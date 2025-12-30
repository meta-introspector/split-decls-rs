// Generated macro for is_hex (function)
macro_rules! Depcrate_char_traitsis_hex {
() => {
// Module: crate::char_traits
// Provides: {"is_hex"}
// Dependencies: {}
# [doc = " Check whether the character is a hexadecimal character (case insensitive)."] # [inline] pub (crate) fn is_hex (c : char) -> bool { c . is_ascii_digit () || ('a' ..= 'f') . contains (& c) || ('A' ..= 'F') . contains (& c) }
};
}
