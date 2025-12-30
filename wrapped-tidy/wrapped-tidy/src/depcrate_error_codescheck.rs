// Generated macro for check (function)
macro_rules! Depcrate_error_codescheck {
() => {
// Module: crate::error_codes
// Provides: {"check"}
// Dependencies: {}
pub fn check (root_path : & Path , search_paths : & [& Path] , verbose : bool , ci_info : & crate :: CiInfo , bad : & mut bool ,) { let mut errors = Vec :: new () ; check_removed_error_code_explanation (ci_info , bad) ; let error_codes = extract_error_codes (root_path , & mut errors) ; if verbose { println ! ("Found {} error codes" , error_codes . len ()) ; println ! ("Highest error code: `{}`" , error_codes . iter () . max () . unwrap ()) ; } let no_longer_emitted = check_error_codes_docs (root_path , & error_codes , & mut errors , verbose) ; check_error_codes_tests (root_path , & error_codes , & mut errors , verbose , & no_longer_emitted) ; check_error_codes_used (search_paths , & error_codes , & mut errors , & no_longer_emitted , verbose) ; for error in errors { tidy_error ! (bad , "{}" , error) ; } }
};
}
