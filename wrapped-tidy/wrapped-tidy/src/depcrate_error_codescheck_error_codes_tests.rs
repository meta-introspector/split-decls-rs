// Generated macro for check_error_codes_tests (function)
macro_rules! Depcrate_error_codescheck_error_codes_tests {
() => {
// Module: crate::error_codes
// Provides: {"check_error_codes_tests"}
// Dependencies: {}
fn check_error_codes_tests (root_path : & Path , error_codes : & [String] , errors : & mut Vec < String > , verbose : bool , no_longer_emitted : & [String] ,) { let tests_path = root_path . join (Path :: new (ERROR_TESTS_PATH)) ; for code in error_codes { let test_path = tests_path . join (format ! ("{code}.stderr")) ; if ! test_path . exists () && ! IGNORE_UI_TEST_CHECK . contains (& code . as_str ()) { verbose_print ! (verbose , "warning: Error code `{code}` needs to have at least one UI test in the `tests/error-codes/` directory`!") ; continue ; } if IGNORE_UI_TEST_CHECK . contains (& code . as_str ()) { if test_path . exists () { errors . push (format ! ("Error code `{code}` has a UI test in `tests/ui/error-codes/{code}.rs`, it shouldn't be listed in `EXEMPTED_FROM_TEST`!")) ; } continue ; } let file = match fs :: read_to_string (& test_path) { Ok (file) => file , Err (err) => { verbose_print ! (verbose , "warning: Failed to read UI test file (`{}`) for `{code}` but the file exists. The test is assumed to work:\n{err}" , test_path . display ()) ; continue ; } } ; if no_longer_emitted . contains (code) { continue ; } let mut found_code = false ; for line in file . lines () { let s = line . trim () ; if s . starts_with ("error[E") && & s [6 .. 11] == code { found_code = true ; break ; } ; } if ! found_code { verbose_print ! (verbose , "warning: Error code `{code}` has a UI test file, but doesn't contain its own error code!") ; } } }
};
}
