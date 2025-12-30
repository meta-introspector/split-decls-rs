// Generated macro for checkstyle_test (function)
macro_rules! Depcrate_testcheckstyle_test {
() => {
// Module: crate::test
// Provides: {"checkstyle_test"}
// Dependencies: {}
# [test] fn checkstyle_test () { init_log () ; let filename = "tests/writemode/source/fn-single-line.rs" ; let expected_filename = "tests/writemode/target/checkstyle.xml" ; assert_output (Path :: new (filename) , Path :: new (expected_filename)) ; }
};
}
