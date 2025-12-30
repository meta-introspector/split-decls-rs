// Generated macro for invalid_continue_outside_loop (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_continue_outside_loop {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_continue_outside_loop"}
// Dependencies: {}
# [test] fn invalid_continue_outside_loop () { assert_err_msg (r#"{% continue %}"# , & ["1:1" , "{% continue %}" , "expected a template"]) ; }
};
}
