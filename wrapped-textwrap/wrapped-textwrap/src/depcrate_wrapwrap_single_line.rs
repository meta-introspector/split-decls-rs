// Generated macro for wrap_single_line (function)
macro_rules! Depcrate_wrapwrap_single_line {
() => {
// Module: crate::wrap
// Provides: {"wrap_single_line"}
// Dependencies: {}
pub (crate) fn wrap_single_line < 'a > (line : & 'a str , options : & Options < '_ > , lines : & mut Vec < Cow < 'a , str > > ,) { let indent = if lines . is_empty () { options . initial_indent } else { options . subsequent_indent } ; if line . len () < options . width && indent . is_empty () { if options . preserve_trailing_space { lines . push (Cow :: from (line)) ; } else { lines . push (Cow :: from (line . trim_end_matches (' '))) ; } } else { wrap_single_line_slow_path (line , options , lines) } }
};
}
