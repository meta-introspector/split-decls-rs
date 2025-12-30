// Generated macro for is_uri_char (function)
macro_rules! Depcrate_char_traitsis_uri_char {
() => {
// Module: crate::char_traits
// Provides: {"is_uri_char"}
// Dependencies: {}
# [doc = " Check whether the character is a valid URI character."] # [inline] pub (crate) fn is_uri_char (c : char) -> bool { is_word_char (c) || "#;/?:@&=+$,_.!~*\'()[]%" . contains (c) }
};
}
