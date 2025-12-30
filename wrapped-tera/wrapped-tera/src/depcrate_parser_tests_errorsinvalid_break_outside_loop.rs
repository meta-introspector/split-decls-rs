// Generated macro for invalid_break_outside_loop (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_break_outside_loop {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_break_outside_loop"}
// Dependencies: {}
# [test] fn invalid_break_outside_loop () { assert_err_msg (r#"{% break %}"# , & ["1:1" , "{% break %}" , "expected a template"]) ; }
};
}
