// Generated macro for string_eq_ignore_newline_repr (function)
macro_rules! Depcrate_teststring_eq_ignore_newline_repr {
() => {
// Module: crate::test
// Provides: {"string_eq_ignore_newline_repr"}
// Dependencies: {}
fn string_eq_ignore_newline_repr (left : & str , right : & str) -> bool { let left = CharsIgnoreNewlineRepr (left . chars () . peekable ()) ; let right = CharsIgnoreNewlineRepr (right . chars () . peekable ()) ; left . eq (right) }
};
}
