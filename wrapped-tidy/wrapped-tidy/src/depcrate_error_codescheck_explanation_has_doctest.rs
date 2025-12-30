// Generated macro for check_explanation_has_doctest (function)
macro_rules! Depcrate_error_codescheck_explanation_has_doctest {
() => {
// Module: crate::error_codes
// Provides: {"check_explanation_has_doctest"}
// Dependencies: {}
# [doc = " This function returns a tuple indicating whether the provided explanation:"] # [doc = " a) has a code example, tested or not."] # [doc = " b) has a valid doctest"] fn check_explanation_has_doctest (explanation : & str , err_code : & str) -> (bool , bool , bool , bool) { let mut found_code_example = false ; let mut found_proper_doctest = false ; let mut emit_ignore_warning = false ; let mut no_longer_emitted = false ; for line in explanation . lines () { let line = line . trim () ; if line . starts_with ("```") { found_code_example = true ; if line . contains ("compile_fail") && line . contains (err_code) { found_proper_doctest = true ; } if line . contains ("ignore") { emit_ignore_warning = true ; found_proper_doctest = true ; } } else if line . starts_with ("#### Note: this error code is no longer emitted by the compiler") { no_longer_emitted = true ; found_code_example = true ; found_proper_doctest = true ; } } (found_code_example , found_proper_doctest , emit_ignore_warning , no_longer_emitted) }
};
}
