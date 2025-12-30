// Generated macro for is_blank_or_breakz (function)
macro_rules! Depcrate_char_traitsis_blank_or_breakz {
() => {
// Module: crate::char_traits
// Provides: {"is_blank_or_breakz"}
// Dependencies: {}
# [doc = " Check whether the character is nil, a linebreak or a whitespace."] # [doc = ""] # [doc = " `\\0`, ` `, `\\t`, `\\n`, `\\r`"] # [inline] pub (crate) fn is_blank_or_breakz (c : char) -> bool { is_blank (c) || is_breakz (c) }
};
}
