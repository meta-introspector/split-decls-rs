// Generated macro for format_files_find_new_files_via_cfg_match (function)
macro_rules! Depcrate_testformat_files_find_new_files_via_cfg_match {
() => {
// Module: crate::test
// Provides: {"format_files_find_new_files_via_cfg_match"}
// Dependencies: {}
# [test] fn format_files_find_new_files_via_cfg_match () { init_log () ; run_test_with (& TestSetting :: default () , | | { let files = vec ! [Path :: new ("tests/source/cfg_match/lib2.rs") , Path :: new ("tests/source/cfg_match/lib.rs") ,] ; let config = Config :: default () ; let mut session = Session :: < io :: Stdout > :: new (config , None) ; let mut write_result = HashMap :: new () ; for file in files { assert ! (file . exists ()) ; let result = session . format (Input :: File (file . into ())) . unwrap () ; assert ! (! session . has_formatting_errors ()) ; assert ! (! result . has_warnings ()) ; let mut source_file = SourceFile :: new () ; mem :: swap (& mut session . source_file , & mut source_file) ; for (filename , text) in source_file { if let FileName :: Real (ref filename) = filename { write_result . insert (filename . to_owned () , text) ; } } } assert_eq ! (6 , write_result . len () , "Should have uncovered an extra file (format_me_please_x.rs) via lib.rs") ; assert ! (handle_result (write_result , None) . is_ok ()) ; }) ; }
};
}
