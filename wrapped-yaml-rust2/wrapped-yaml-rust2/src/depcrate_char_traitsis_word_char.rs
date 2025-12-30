// Generated macro for is_word_char (function)
macro_rules! Depcrate_char_traitsis_word_char {
() => {
// Module: crate::char_traits
// Provides: {"is_word_char"}
// Dependencies: {}
# [doc = " Check whether the character is a valid word character."] # [inline] pub (crate) fn is_word_char (c : char) -> bool { is_alpha (c) && c != '_' }
};
}
