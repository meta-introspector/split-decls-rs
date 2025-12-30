// Generated macro for json_test (function)
macro_rules! Depcrate_testjson_test {
() => {
// Module: crate::test
// Provides: {"json_test"}
// Dependencies: {}
# [test] fn json_test () { init_log () ; let filename = "tests/writemode/source/json.rs" ; let expected_filename = "tests/writemode/target/output.json" ; assert_output (Path :: new (filename) , Path :: new (expected_filename)) ; }
};
}
