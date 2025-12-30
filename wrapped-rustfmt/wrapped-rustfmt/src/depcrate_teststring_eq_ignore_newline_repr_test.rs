// Generated macro for string_eq_ignore_newline_repr_test (function)
macro_rules! Depcrate_teststring_eq_ignore_newline_repr_test {
() => {
// Module: crate::test
// Provides: {"string_eq_ignore_newline_repr_test"}
// Dependencies: {}
# [test] fn string_eq_ignore_newline_repr_test () { init_log () ; assert ! (string_eq_ignore_newline_repr ("" , "")) ; assert ! (! string_eq_ignore_newline_repr ("" , "abc")) ; assert ! (! string_eq_ignore_newline_repr ("abc" , "")) ; assert ! (string_eq_ignore_newline_repr ("a\nb\nc\rd" , "a\nb\r\nc\rd")) ; assert ! (string_eq_ignore_newline_repr ("a\r\n\r\n\r\nb" , "a\n\n\nb")) ; assert ! (! string_eq_ignore_newline_repr ("a\r\nbcd" , "a\nbcdefghijk")) ; }
};
}
