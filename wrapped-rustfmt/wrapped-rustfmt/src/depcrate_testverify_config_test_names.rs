// Generated macro for verify_config_test_names (function)
macro_rules! Depcrate_testverify_config_test_names {
() => {
// Module: crate::test
// Provides: {"verify_config_test_names"}
// Dependencies: {}
# [test] fn verify_config_test_names () { init_log () ; for path in & [Path :: new ("tests/source/configs") , Path :: new ("tests/target/configs") ,] { for entry in fs :: read_dir (path) . expect ("couldn't read configs directory") { let entry = entry . expect ("couldn't get directory entry") ; let path = entry . path () ; if path . is_dir () { let config_name = path . file_name () . unwrap () . to_str () . unwrap () ; verify_config_used (& path , config_name) ; } } } }
};
}
