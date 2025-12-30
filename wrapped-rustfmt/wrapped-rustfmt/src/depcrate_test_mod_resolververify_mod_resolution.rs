// Generated macro for verify_mod_resolution (function)
macro_rules! Depcrate_test_mod_resolververify_mod_resolution {
() => {
// Module: crate::test::mod_resolver
// Provides: {"verify_mod_resolution"}
// Dependencies: {}
fn verify_mod_resolution (input_file_name : & str , exp_misformatted_files : & [& str]) { let input_file = PathBuf :: from (input_file_name) ; let config = read_config (& input_file) ; let mut session = Session :: < io :: Stdout > :: new (config , None) ; let report = session . format (Input :: File (input_file_name . into ())) . expect ("Should not have had any execution errors") ; let errors_by_file = & report . internal . borrow () . 0 ; for exp_file in exp_misformatted_files { assert ! (errors_by_file . contains_key (& FileName :: Real (PathBuf :: from (exp_file)))) ; } }
};
}
