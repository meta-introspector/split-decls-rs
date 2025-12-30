// Generated macro for is_break (function)
macro_rules! Depcrate_char_traitsis_break {
() => {
// Module: crate::char_traits
// Provides: {"is_break"}
// Dependencies: {}
# [doc = " Check whether the character is a line break (`\\r` or `\\n`)."] # [inline] pub (crate) fn is_break (c : char) -> bool { c == '\n' || c == '\r' }
};
}
