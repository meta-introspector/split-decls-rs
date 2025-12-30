// Generated macro for invalid_import_macros_missing_namespace (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_import_macros_missing_namespace {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_import_macros_missing_namespace"}
// Dependencies: {}
# [test] fn invalid_import_macros_missing_namespace () { assert_err_msg (r#"{% import "hello" as %}"# , & ["1:22" , "expected an identifier (must start with a-z)"] ,) ; }
};
}
