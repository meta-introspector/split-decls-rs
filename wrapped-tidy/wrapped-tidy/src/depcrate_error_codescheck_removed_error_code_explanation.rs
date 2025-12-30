// Generated macro for check_removed_error_code_explanation (function)
macro_rules! Depcrate_error_codescheck_removed_error_code_explanation {
() => {
// Module: crate::error_codes
// Provides: {"check_removed_error_code_explanation"}
// Dependencies: {}
fn check_removed_error_code_explanation (ci_info : & crate :: CiInfo , bad : & mut bool) { let Some (base_commit) = & ci_info . base_commit else { eprintln ! ("Skipping error code explanation removal check") ; return ; } ; let Some (diff) = crate :: git_diff (base_commit , "--name-status") else { * bad = true ; eprintln ! ("removed error code explanation tidy check: Failed to run git diff") ; return ; } ; if diff . lines () . any (| line | { line . starts_with ('D') && line . contains ("compiler/rustc_error_codes/src/error_codes/") }) { * bad = true ; eprintln ! ("tidy check error: Error code explanations should never be removed!") ; eprintln ! ("Take a look at E0001 to see how to handle it.") ; return ; } println ! ("No error code explanation was removed!") ; }
};
}
