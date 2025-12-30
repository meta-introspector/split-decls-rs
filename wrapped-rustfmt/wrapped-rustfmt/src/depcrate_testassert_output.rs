// Generated macro for assert_output (function)
macro_rules! Depcrate_testassert_output {
() => {
// Module: crate::test
// Provides: {"assert_output"}
// Dependencies: {}
fn assert_output (source : & Path , expected_filename : & Path) { let config = read_config (source) ; let (_ , source_file , _) = format_file (source , config . clone ()) ; let mut out = vec ! [] ; let _ = source_file :: write_all_files (& source_file , & mut out , & config) ; let output = String :: from_utf8 (out) . unwrap () ; let mut expected_file = fs :: File :: open (& expected_filename) . expect ("couldn't open target") ; let mut expected_text = String :: new () ; expected_file . read_to_string (& mut expected_text) . expect ("Failed reading target") ; let compare = make_diff (& expected_text , & output , DIFF_CONTEXT_SIZE) ; if ! compare . is_empty () { let mut failures = HashMap :: new () ; failures . insert (source . to_owned () , compare) ; print_mismatches_default_message (failures) ; panic ! ("Text does not match expected output") ; } }
};
}
