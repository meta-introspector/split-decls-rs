// Generated macro for is_breakz (function)
macro_rules! Depcrate_char_traitsis_breakz {
() => {
// Module: crate::char_traits
// Provides: {"is_breakz"}
// Dependencies: {}
# [doc = " Check whether the character is nil or a line break (`\\0`, `\\r`, `\\n`)."] # [inline] pub (crate) fn is_breakz (c : char) -> bool { is_break (c) || is_z (c) }
};
}
