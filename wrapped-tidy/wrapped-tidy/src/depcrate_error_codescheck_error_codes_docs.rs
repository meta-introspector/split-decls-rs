// Generated macro for check_error_codes_docs (function)
macro_rules! Depcrate_error_codescheck_error_codes_docs {
() => {
// Module: crate::error_codes
// Provides: {"check_error_codes_docs"}
// Dependencies: {}
# [doc = " Stage 2: Checks that long-form error code explanations exist and have doctests."] fn check_error_codes_docs (root_path : & Path , error_codes : & [String] , errors : & mut Vec < String > , verbose : bool ,) -> Vec < String > { let docs_path = root_path . join (Path :: new (ERROR_DOCS_PATH)) ; let mut no_longer_emitted_codes = Vec :: new () ; walk (& docs_path , | _ , _ | false , & mut | entry , contents | { let path = entry . path () ; if path . extension () != Some (OsStr :: new ("md")) { errors . push (format ! ("Found unexpected non-markdown file in error code docs directory: {}" , path . display ())) ; return ; } let filename = path . file_name () . unwrap () . to_str () . unwrap () . split_once ('.') ; let err_code = filename . unwrap () . 0 ; if error_codes . iter () . all (| e | e != err_code) { errors . push (format ! ("Found valid file `{}` in error code docs directory without corresponding \
                entry in `rustc_error_codes/src/lib.rs`" , path . display ())) ; return ; } let (found_code_example , found_proper_doctest , emit_ignore_warning , no_longer_emitted) = check_explanation_has_doctest (contents , err_code) ; if emit_ignore_warning { verbose_print ! (verbose , "warning: Error code `{err_code}` uses the ignore header. This should not be used, add the error code to the \
                `IGNORE_DOCTEST_CHECK` constant instead.") ; } if no_longer_emitted { no_longer_emitted_codes . push (err_code . to_owned ()) ; } if ! found_code_example { verbose_print ! (verbose , "warning: Error code `{err_code}` doesn't have a code example, all error codes are expected to have one \
                (even if untested).") ; return ; } let test_ignored = IGNORE_DOCTEST_CHECK . contains (& err_code) ; if ! found_proper_doctest && ! test_ignored { errors . push (format ! ("`{}` doesn't use its own error code in compile_fail example" , path . display () ,)) ; } else if found_proper_doctest && test_ignored { errors . push (format ! ("`{}` has a compile_fail doctest with its own error code, it shouldn't \
                be listed in `IGNORE_DOCTEST_CHECK`" , path . display () ,)) ; } }) ; no_longer_emitted_codes }
};
}
