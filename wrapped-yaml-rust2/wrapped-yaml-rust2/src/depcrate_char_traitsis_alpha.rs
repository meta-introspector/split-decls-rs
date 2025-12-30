// Generated macro for is_alpha (function)
macro_rules! Depcrate_char_traitsis_alpha {
() => {
// Module: crate::char_traits
// Provides: {"is_alpha"}
// Dependencies: {}
# [doc = " Check whether the character is a digit, letter, `_` or `-`."] # [inline] pub (crate) fn is_alpha (c : char) -> bool { matches ! (c , '0' ..='9' | 'a' ..='z' | 'A' ..='Z' | '_' | '-') }
};
}
