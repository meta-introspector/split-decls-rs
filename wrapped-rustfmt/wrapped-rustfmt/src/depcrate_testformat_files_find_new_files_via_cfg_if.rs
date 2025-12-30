// Generated macro for format_files_find_new_files_via_cfg_if (function)
macro_rules! Depcrate_testformat_files_find_new_files_via_cfg_if {
() => {
// Module: crate::test
// Provides: {"format_files_find_new_files_via_cfg_if"}
// Dependencies: {}
# [test] fn format_files_find_new_files_via_cfg_if () { init_log () ; run_test_with (& TestSetting :: default () , | | { let files = vec ! [Path :: new ("tests/source/issue-4656/lib2.rs") , Path :: new ("tests/source/issue-4656/lib.rs") ,] ; let config = Config :: default () ; let mut session = Session :: < io :: Stdout > :: new (config , None) ; let mut write_result = HashMap :: new () ; for file in files { assert ! (file . exists ()) ; let result = session . format (Input :: File (file . into ())) . unwrap () ; assert ! (! session . has_formatting_errors ()) ; assert ! (! result . has_warnings ()) ; let mut source_file = SourceFile :: new () ; mem :: swap (& mut session . source_file , & mut source_file) ; for (filename , text) in source_file { if let FileName :: Real (ref filename) = filename { write_result . insert (filename . to_owned () , text) ; } } } assert_eq ! (3 , write_result . len () , "Should have uncovered an extra file (format_me_please.rs) via lib.rs") ; assert ! (handle_result (write_result , None) . is_ok ()) ; }) ; }
};
}
