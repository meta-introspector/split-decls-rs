// Generated macro for is_tag_char (function)
macro_rules! Depcrate_char_traitsis_tag_char {
() => {
// Module: crate::char_traits
// Provides: {"is_tag_char"}
// Dependencies: {}
# [doc = " Check whether the character is a valid tag character."] # [inline] pub (crate) fn is_tag_char (c : char) -> bool { is_uri_char (c) && ! is_flow (c) && c != '!' }
};
}
