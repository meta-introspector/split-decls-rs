// Generated macro for invalid_import_macros_missing_filename (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_import_macros_missing_filename {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_import_macros_missing_filename"}
// Dependencies: {}
# [test] fn invalid_import_macros_missing_filename () { assert_err_msg ("{% import as macros %}" , & ["1:11" , "expected a string"]) ; }
};
}
