// Generated macro for macro_style (function)
macro_rules! Depcrate_macrosmacro_style {
() => {
// Module: crate::macros
// Provides: {"macro_style"}
// Dependencies: {}
pub (crate) fn macro_style (mac : & ast :: MacCall , context : & RewriteContext < '_ >) -> Delimiter { let snippet = context . snippet (mac . span ()) ; let paren_pos = snippet . find_uncommented ("(") . unwrap_or (usize :: MAX) ; let bracket_pos = snippet . find_uncommented ("[") . unwrap_or (usize :: MAX) ; let brace_pos = snippet . find_uncommented ("{") . unwrap_or (usize :: MAX) ; if paren_pos < bracket_pos && paren_pos < brace_pos { Delimiter :: Parenthesis } else if bracket_pos < brace_pos { Delimiter :: Bracket } else { Delimiter :: Brace } }
};
}
