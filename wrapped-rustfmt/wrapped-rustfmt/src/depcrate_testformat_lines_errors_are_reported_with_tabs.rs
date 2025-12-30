// Generated macro for format_lines_errors_are_reported_with_tabs (function)
macro_rules! Depcrate_testformat_lines_errors_are_reported_with_tabs {
() => {
// Module: crate::test
// Provides: {"format_lines_errors_are_reported_with_tabs"}
// Dependencies: {}
# [test] fn format_lines_errors_are_reported_with_tabs () { init_log () ; let long_identifier = String :: from_utf8 (vec ! [b'a' ; 97]) . unwrap () ; let input = Input :: Text (format ! ("fn a() {{\n\t{long_identifier}\n}}")) ; let mut config = Config :: default () ; config . set () . error_on_line_overflow (true) ; config . set () . hard_tabs (true) ; let mut session = Session :: < io :: Stdout > :: new (config , None) ; session . format (input) . unwrap () ; assert ! (session . has_formatting_errors ()) ; }
};
}
