// Generated macro for format_lines_errors_are_reported (function)
macro_rules! Depcrate_testformat_lines_errors_are_reported {
() => {
// Module: crate::test
// Provides: {"format_lines_errors_are_reported"}
// Dependencies: {}
# [test] fn format_lines_errors_are_reported () { init_log () ; let long_identifier = String :: from_utf8 (vec ! [b'a' ; 239]) . unwrap () ; let input = Input :: Text (format ! ("fn {long_identifier}() {{}}")) ; let mut config = Config :: default () ; config . set () . error_on_line_overflow (true) ; let mut session = Session :: < io :: Stdout > :: new (config , None) ; session . format (input) . unwrap () ; assert ! (session . has_formatting_errors ()) ; }
};
}
